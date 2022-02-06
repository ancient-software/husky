use std::path;

use crate::*;
use itertools::Itertools;
use stdx::sync::ARwLock;
use unique_allocator::UniqueAllocatorPtr;

pub trait LiveFiles: AllocateUniqueFile {
    fn get_live_files(&self) -> &ARwLock<HashMap<FilePtr, ARwLock<String>>>;
    fn did_change_source(&mut self, id: FilePtr);

    fn set_live_file_text(&mut self, path: PathBuf, text: String) {
        let id = self.alloc_file(path);
        self.get_live_files()
            .write(|live_docs| live_docs.insert(id, ARwLock::new(text)));
        self.did_change_source(id);
    }

    fn apply_live_file_changes(
        &mut self,
        path: PathBuf,
        changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) {
        let id = self.alloc_file(path);
        self.get_live_files().write(|live_docs| {
            live_docs
                .get(&id)
                .expect("what")
                .write(|text| utils::apply_document_changes(text, changes))
        });
        self.did_change_source(id);
    }
}

#[salsa::query_group(FileQueryStorage)]
pub trait FileSalsaQuery: LiveFiles {
    fn file_content(&self, id: FilePtr) -> FileContent;

    fn main_file_id(&self, module_file_id: FilePtr) -> Option<FilePtr>;
}

fn file_content(this: &dyn FileSalsaQuery, id: FilePtr) -> FileContent {
    this.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    this.get_live_files()
        .read(|live_docs| match live_docs.get(&id) {
            Some(text) => FileContent::Live(text.clone_to_arc()),
            None => {
                let pth: PathBuf = (*id).into();
                if pth.is_file() {
                    FileContent::OnDisk(Arc::new(std::fs::read_to_string(pth).expect("io failure")))
                } else {
                    FileContent::NonExistent
                }
            }
        })
}

fn main_file_id(this: &dyn FileSalsaQuery, module_file_id: FilePtr) -> Option<FilePtr> {
    let pth: PathBuf = (*module_file_id).into();
    for ancestor in pth.ancestors() {
        let id = this.alloc_file(ancestor.with_file_name("main.hsk"));
        match this.file_content(id) {
            FileContent::OnDisk(_) | FileContent::Live(_) => return Some(id),
            FileContent::Deleted | FileContent::NonExistent => (),
        }
    }
    None
}

pub trait FileQuery: FileSalsaQuery {
    fn file_exists(&self, id: FilePtr) -> bool {
        match self.file_content(id) {
            FileContent::OnDisk(_) => true,
            FileContent::Live(_) => true,
            FileContent::Deleted => false,
            FileContent::NonExistent => false,
        }
    }

    fn all_main_files(&self) -> Vec<FilePtr> {
        self.file_unique_allocator()
            .id_iter()
            .filter_map(|id| self.main_file_id(id))
            .unique()
            .collect()
    }

    fn text(&self, id: FilePtr) -> Option<Arc<String>> {
        match self.file_content(id) {
            FileContent::OnDisk(text) => Some(text),
            FileContent::Live(text) => Some(text),
            FileContent::Deleted => None,
            FileContent::NonExistent => None,
        }
    }

    fn url(&self, id: FilePtr) -> lsp_types::Url {
        return url_from_abs_path(&id);

        pub(crate) fn url_from_abs_path(path: &Path) -> lsp_types::Url {
            let url = lsp_types::Url::from_file_path(path).unwrap();
            match path.components().next() {
                Some(path::Component::Prefix(prefix))
                    if matches!(
                        prefix.kind(),
                        path::Prefix::Disk(_) | path::Prefix::VerbatimDisk(_)
                    ) =>
                {
                    // Need to lowercase driver letter
                }
                _ => return url,
            }

            let driver_letter_range = {
                let (scheme, drive_letter, _rest) =
                    match url.as_str().splitn(3, ':').collect_tuple() {
                        Some(it) => it,
                        None => return url,
                    };
                let start = scheme.len() + ':'.len_utf8();
                start..(start + drive_letter.len())
            };

            // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
            // machinery *also* canonicalizes the drive letter. So, just massage the
            // string in place.
            let mut url: String = url.into();
            url[driver_letter_range].make_ascii_lowercase();
            lsp_types::Url::parse(&url).unwrap()
        }
    }
}
