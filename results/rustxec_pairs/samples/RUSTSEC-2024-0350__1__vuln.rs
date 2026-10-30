pub(crate) fn open_from_paths(
        mut git_dir: PathBuf,
        mut worktree_dir: Option<PathBuf>,
        mut options: Options,
    ) -> Result<Self, Error> {
        let _span = gix_trace::detail!("open_from_paths()");
        let Options {
            git_dir_trust,
            object_store_slots,
            filter_config_section,
            lossy_config,
            lenient_config,
            bail_if_untrusted,
            open_path_as_is: _,
            permissions:
                Permissions {
                    ref env,
                    config,
                    attributes,
                },
            ref api_config_overrides,
            ref cli_config_overrides,
            ref mut current_dir,
        } = options;
        let git_dir_trust = git_dir_trust.expect("trust must be determined by now");

        let mut common_dir = gix_discover::path::from_plain_file(git_dir.join("commondir").as_ref())
            .transpose()?
            .map(|cd| git_dir.join(cd));
        let repo_config = config::cache::StageOne::new(
            common_dir.as_deref().unwrap_or(&git_dir),
            git_dir.as_ref(),
            git_dir_trust,
            lossy_config,
            lenient_config,
        )?;

        if repo_config.precompose_unicode {
            git_dir = gix_utils::str::precompose_path(git_dir.into()).into_owned();
            if let Some(common_dir) = common_dir.as_mut() {
                if let Cow::Owned(precomposed) = gix_utils::str::precompose_path((&*common_dir).into()) {
                    *common_dir = precomposed;
                }
            }
            if let Some(worktree_dir) = worktree_dir.as_mut() {
                if let Cow::Owned(precomposed) = gix_utils::str::precompose_path((&*worktree_dir).into()) {
                    *worktree_dir = precomposed;
                }
            }
        }
        let common_dir_ref = common_dir.as_deref().unwrap_or(&git_dir);

        let current_dir = {
            let current_dir_ref = current_dir.as_mut().expect("BUG: current_dir must be set by caller");
            if repo_config.precompose_unicode {
                if let Cow::Owned(precomposed) = gix_utils::str::precompose_path((&*current_dir_ref).into()) {
                    *current_dir_ref = precomposed;
                }
            }
            current_dir_ref.as_path()
        };

        let mut refs = {
            let reflog = repo_config.reflog.unwrap_or(gix_ref::store::WriteReflog::Disable);
            let object_hash = repo_config.object_hash;
            match &common_dir {
                Some(common_dir) => crate::RefStore::for_linked_worktree(
                    git_dir.to_owned(),
                    common_dir.into(),
                    reflog,
                    object_hash,
                    repo_config.precompose_unicode,
                ),
                None => crate::RefStore::at(git_dir.to_owned(), reflog, object_hash, repo_config.precompose_unicode),
            }
        };
        let head = refs.find("HEAD").ok();
        let git_install_dir = crate::path::install_dir().ok();
        let home = gix_path::env::home_dir().and_then(|home| env.home.check_opt(home));

        let mut filter_config_section = filter_config_section.unwrap_or(config::section::is_trusted);
        let config = config::Cache::from_stage_one(
            repo_config,
            common_dir_ref,
            head.as_ref().and_then(|head| head.target.try_name()),
            filter_config_section,
            git_install_dir.as_deref(),
            home.as_deref(),
            *env,
            attributes,
            config,
            lenient_config,
            api_config_overrides,
            cli_config_overrides,
        )?;

        if bail_if_untrusted && git_dir_trust != gix_sec::Trust::Full {
            check_safe_directories(
                &git_dir,
                git_install_dir.as_deref(),
                current_dir,
                home.as_deref(),
                &config,
            )?;
        }

        // core.worktree might be used to overwrite the worktree directory
        if !config.is_bare {
            if let Some(wt) = config
                .resolved
                .path_filter("core", None, Core::WORKTREE.name, &mut filter_config_section)
            {
                let wt_clone = wt.clone();
                let wt_path = wt
                    .interpolate(interpolate_context(git_install_dir.as_deref(), home.as_deref()))
                    .map_err(|err| config::Error::PathInterpolation {
                        path: wt_clone.value.into_owned(),
                        source: err,
                    })?;
                worktree_dir = gix_path::normalize(git_dir.join(wt_path).into(), current_dir).map(Cow::into_owned);
                #[allow(unused_variables)]
                if let Some(worktree_path) = worktree_dir.as_deref().filter(|wtd| !wtd.is_dir()) {
                    gix_trace::warn!("The configured worktree path '{}' is not a directory or doesn't exist - `core.worktree` may be misleading", worktree_path.display());
                }
            } else if !config.lenient_config
                && config
                    .resolved
                    .boolean_filter("core", None, Core::WORKTREE.name, &mut filter_config_section)
                    .is_some()
            {
                return Err(Error::from(config::Error::ConfigTypedString(
                    config::key::GenericErrorWithValue::from(&Core::WORKTREE),
                )));
            }
        }

        match worktree_dir {
            None if !config.is_bare => {
                worktree_dir = Some(git_dir.parent().expect("parent is always available").to_owned());
            }
            Some(_) => {
                // note that we might be bare even with a worktree directory - work trees don't have to be
                // the parent of a non-bare repository.
            }
            None => {}
        }

        refs.write_reflog = config::cache::util::reflog_or_default(config.reflog, worktree_dir.is_some());
        refs.namespace.clone_from(&config.refs_namespace);
        let replacements = replacement_objects_refs_prefix(&config.resolved, lenient_config, filter_config_section)?
            .and_then(|prefix| {
                let _span = gix_trace::detail!("find replacement objects");
                let platform = refs.iter().ok()?;
                let iter = platform.prefixed(&prefix).ok()?;
                let prefix = prefix.to_str()?;
                let replacements = iter
                    .filter_map(Result::ok)
                    .filter_map(|r: gix_ref::Reference| {
                        let target = r.target.try_id()?.to_owned();
                        let source =
                            gix_hash::ObjectId::from_hex(r.name.as_bstr().strip_prefix(prefix.as_bytes())?).ok()?;
                        Some((source, target))
                    })
                    .collect::<Vec<_>>();
                Some(replacements)
            })
            .unwrap_or_default();

        Ok(ThreadSafeRepository {
            objects: OwnShared::new(gix_odb::Store::at_opts(
                common_dir_ref.join("objects"),
                &mut replacements.into_iter(),
                gix_odb::store::init::Options {
                    slots: object_store_slots,
                    object_hash: config.object_hash,
                    use_multi_pack_index: config.use_multi_pack_index,
                    current_dir: current_dir.to_owned().into(),
                },
            )?),
            common_dir,
            refs,
            work_tree: worktree_dir,
            config,
            // used when spawning new repositories off this one when following worktrees
            linked_worktree_options: options,
            #[cfg(feature = "index")]
            index: gix_fs::SharedFileSnapshotMut::new().into(),
            shallow_commits: gix_fs::SharedFileSnapshotMut::new().into(),
            #[cfg(feature = "attributes")]
            modules: gix_fs::SharedFileSnapshotMut::new().into(),
        })
    }