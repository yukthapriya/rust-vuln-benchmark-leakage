// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0435
// CVE: N/A
// CWE: N/A
// Crate: fyrox-core
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/fyrox-core
// Extraction Method: repo_search
// GitHub Repo: FyroxEngine/Fyrox
// Commit: 0c0ae47
// File: fyrox-ui/src/file_browser/mod.rs


BaseSceneGraph,
 };
 use notify::Watcher;
 use std::{
     borrow::BorrowMut,
     cmp::Ordering,
     path_item: Handle<UiNode>,
 }
 
/// Builds entire file system tree to given final_path.
fn build_all(
    root: Option<&PathBuf>,
    final_path: &Path,
    mut filter: Option<Filter>,
    menu: RcUiNodeHandle,
    root_title: Option<&str>,
    ctx: &mut BuildContext,
) -> BuildResult {
    let mut dest_path = PathBuf::new();
    if let Ok(canonical_final_path) = final_path.canonicalize() {
         if let Some(canonical_root) = root.and_then(|r| r.canonicalize().ok()) {
            if let Ok(stripped) = canonical_final_path.strip_prefix(canonical_root) {
                stripped.clone_into(&mut dest_path);
             }
         } else {
            dest_path = canonical_final_path;
         }
     }
 
     // There should be at least one component in the path. If the path is empty, this means that
     // it "points" to the current directory.
    if dest_path.as_os_str().is_empty() {
        dest_path.push(".");
     }
 
     // Relative paths must always start from CurDir component (./), otherwise the root dir will be ignored
     // and the tree will be incorrect.
    if !dest_path.is_absolute() {
        dest_path = Path::new(".").join(dest_path);
     }
 
    let dest_path_components = dest_path.components().collect::<Vec<Component>>();
    #[allow(unused_variables)]
    let dest_disk = dest_path_components.first().and_then(|c| {
        if let Component::Prefix(prefix) = c {
            if let Prefix::Disk(disk_letter) | Prefix::VerbatimDisk(disk_letter) = prefix.kind() {
                Some(disk_letter)
             } else {
                None
             }
         } else {
            None
        }
    });
 
    let mut root_items = Vec::new();
    let mut parent = if let Some(root) = root {
        let path = if std::env::current_dir().is_ok_and(|dir| &dir == root) {
            Path::new(".")
        } else {
            root.as_path()
        };
        let item = build_tree_item(path, Path::new(""), menu.clone(), true, ctx, root_title);
        root_items.push(item);
        item
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut parent = Handle::NONE;

            // Create items for disks.
            for disk in sysinfo::System::new_with_specifics(RefreshKind::new().with_disks_list())
                .disks()
                .iter()
                .map(|i| i.mount_point().to_string_lossy())
            {
                let disk_letter = disk.chars().next().unwrap() as u8;
                 let is_disk_part_of_path = dest_disk == Some(disk_letter);
 
                 let item = build_tree_item(
                 );
 
                 if is_disk_part_of_path {
                    parent = item;
                 }
 
                root_items.push(item);
             }
 
            parent
         }
 
        #[cfg(target_arch = "wasm32")]
        {
            Handle::NONE
        }
    };
 
     let mut path_item = Handle::NONE;