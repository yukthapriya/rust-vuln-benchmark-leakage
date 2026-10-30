// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0435 (FIXED)
// Crate: fyrox-core
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: FyroxEngine/Fyrox
// Commit: 0c0ae47
// File: fyrox-ui/src/file_browser/mod.rs


BaseSceneGraph,
 };
 use notify::Watcher;
use std::borrow::Cow;
 use std::{
     borrow::BorrowMut,
     cmp::Ordering,
     path_item: Handle<UiNode>,
 }
 
fn disk_letter(components: &[Component]) -> Option<u8> {
    if let Some(Component::Prefix(prefix)) = components.first() {
        if let Prefix::Disk(disk_letter) | Prefix::VerbatimDisk(disk_letter) = prefix.kind() {
            return Some(disk_letter);
        }
    }
    None
}

fn sanitize_path(in_path: &Path, root: Option<&PathBuf>) -> PathBuf {
    let mut out_path = PathBuf::new();

    if let Ok(canonical_in_path) = in_path.canonicalize() {
         if let Some(canonical_root) = root.and_then(|r| r.canonicalize().ok()) {
            if let Ok(stripped) = canonical_in_path.strip_prefix(canonical_root) {
                stripped.clone_into(&mut out_path);
             }
         } else {
            out_path = canonical_in_path;
         }
     }
 
     // There should be at least one component in the path. If the path is empty, this means that
     // it "points" to the current directory.
    if out_path.as_os_str().is_empty() {
        out_path.push(".");
     }
 
     // Relative paths must always start from CurDir component (./), otherwise the root dir will be ignored
     // and the tree will be incorrect.
    if !out_path.is_absolute() {
        out_path = Path::new(".").join(out_path);
     }
 
    out_path
}

#[cfg(not(target_arch = "wasm32"))]
struct DisksProvider {
    sys: sysinfo::System,
}

#[cfg(not(target_arch = "wasm32"))]
impl DisksProvider {
    fn new() -> Self {
        Self {
            sys: sysinfo::System::new_with_specifics(RefreshKind::new().with_disks_list()),
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Cow<str>, u8)> {
        self.sys.disks().iter().map(|disk| {
            let mount_point = disk.mount_point().to_string_lossy();
            let disk_letter = mount_point.chars().next().unwrap() as u8;
            (mount_point, disk_letter)
        })
    }
}

#[cfg(target_arch = "wasm32")]
struct DisksProvider;

#[cfg(target_arch = "wasm32")]
impl DisksProvider {
    fn new() -> Self {
        Self
    }

    fn iter(&self) -> impl Iterator<Item = (Cow<str>, u8)> {
        std::iter::empty()
    }
}

struct RootsCollection {
    items: Vec<Handle<UiNode>>,
    root_item: Handle<UiNode>,
}

impl RootsCollection {
    fn new(
        path_components: &[Component],
        root: Option<&PathBuf>,
        menu: &RcUiNodeHandle,
        root_title: Option<&str>,
        ctx: &mut BuildContext,
    ) -> Self {
        if let Some(root) = root {
            let path = if std::env::current_dir().is_ok_and(|dir| &dir == root) {
                Path::new(".")
             } else {
                root.as_path()
            };

            let root_item =
                build_tree_item(path, Path::new(""), menu.clone(), true, ctx, root_title);
            Self {
                items: vec![root_item],
                root_item,
             }
         } else {
            let dest_disk = disk_letter(path_components);
 
            let mut items = Vec::new();
            let mut root_item = Handle::NONE;

            for (disk, disk_letter) in DisksProvider::new().iter() {
                 let is_disk_part_of_path = dest_disk == Some(disk_letter);
 
                 let item = build_tree_item(
                 );
 
                 if is_disk_part_of_path {
                    root_item = item;
                 }
 
                items.push(item);
             }
 
            Self { items, root_item }
         }
    }
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
    let dest_path = sanitize_path(final_path, root);

    let dest_path_components = dest_path.components().collect::<Vec<Component>>();

    let RootsCollection {
        items: mut root_items,
        root_item: mut parent,
    } = RootsCollection::new(&dest_path_components, root, &menu, root_title, ctx);
 
     let mut path_item = Handle::NONE;