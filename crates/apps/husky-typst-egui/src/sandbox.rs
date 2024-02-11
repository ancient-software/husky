use std::sync::Arc;

use comemo::Prehashed;
use husky_typst::{
    compile,
    diag::{FileError, FileResult},
    eval::Tracer,
    syntax::VirtualPath,
    LibraryBuilder,
};
use husky_typst::{
    foundations::Bytes,
    syntax::{FileId, Source},
};
use husky_typst::{foundations::Datetime, Library};
use husky_typst::{
    text::{TypstFont, TypstFontBook},
    World,
};

pub struct Sandbox {
    library: Prehashed<Library>,
    book: Prehashed<TypstFontBook>,
    fonts: Vec<TypstFont>,
}

fn make_source(source: String) -> Source {
    Source::new(FileId::new_fake(VirtualPath::new("input.typ")), source)
}

fn fonts() -> Vec<TypstFont> {
    use husky_print_utils::p;
    p!(std::env::current_dir());
    std::fs::read_dir("assets/fonts")
        .unwrap()
        .map(Result::unwrap)
        .flat_map(|entry| {
            let bytes = std::fs::read(entry.path()).unwrap();
            let buffer = Bytes::from(bytes);
            TypstFont::iter(buffer)
        })
        .collect()
}

pub struct WithSource {
    sandbox: Arc<Sandbox>,
    source: Source,
}

impl Sandbox {
    pub fn new() -> Self {
        let fonts = fonts();

        Self {
            library: Prehashed::new(LibraryBuilder::default().build()),
            book: Prehashed::new(TypstFontBook::from_fonts(&fonts)),
            fonts,
        }
    }

    pub fn with_source(self: Arc<Self>, source: String) -> WithSource {
        WithSource {
            sandbox: self,
            source: make_source(source),
        }
    }
}

impl WithSource {
    pub fn into_source(self) -> Source {
        self.source
    }
}

impl World for WithSource {
    fn library(&self) -> &Prehashed<Library> {
        &self.sandbox.library
    }

    fn main(&self) -> Source {
        self.source.clone()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        assert_eq!(id, self.source.id());
        Ok(self.source.clone())
    }

    fn book(&self) -> &Prehashed<TypstFontBook> {
        &self.sandbox.book
    }

    fn font(&self, id: usize) -> Option<TypstFont> {
        self.sandbox.fonts.get(id).cloned()
    }

    fn file(&self, path: FileId) -> FileResult<Bytes> {
        // Err(FileError::NotFound(path.into()))
        todo!()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        todo!()
    }
}

#[test]
fn sandbox_works() {
    let sandbox = Sandbox::new();
    let mut tracer = Tracer::new();
    let source = r#"hello world

$ x + x $
"#
    .to_string();
    let compiled = compile(&Arc::new(sandbox).with_source(source), &mut tracer);
    println!("{compiled:?}");
}
