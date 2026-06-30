use crate::fonts::load_fonts;

use std::path::Path;
use std::sync::OnceLock;

use typst::diag::{FileResult, SourceResult, Warned};
use typst::foundations::{Bytes, Datetime, Dict, Duration};
use typst::syntax::{FileId, RootedPath, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Feature, Features, Library, LibraryExt, World};
use typst_kit::datetime::Time;
use typst_kit::downloader::SystemDownloader;
use typst_kit::files::{FileStore, FsRoot, SystemFiles};
use typst_kit::packages::SystemPackages;
use typst_layout::PagedDocument;

pub struct TyndingWorld {
    // Typst asks its World for all ambient state during compilation: the
    // standard library, fonts, files, the main source id, and date/time.
    // Keeping that state here makes the R-facing compiler call self-contained.
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    files: FileStore<SystemFiles>,
    main: FileId,
    time: Time,
}

static DEFAULT_BOOK: OnceLock<LazyHash<FontBook>> = OnceLock::new();
static DEFAULT_BOOK_WITHOUT_SYSTEM: OnceLock<LazyHash<FontBook>> = OnceLock::new();

impl TyndingWorld {
    pub fn new(
        root: &Path,
        input: &Path,
        font_path: Option<&str>,
        ignore_system_fonts: bool,
        inputs: Dict,
    ) -> Result<Self, String> {
        let fonts = load_fonts(font_path, ignore_system_fonts)?;
        let book = build_font_book(&fonts, font_path, ignore_system_fonts);
        let library = Library::builder()
            .with_features(Features::from_iter([Feature::Html]))
            .with_inputs(inputs)
            .build();
        // Typst stores source identities as virtual project paths, not raw file
        // system paths. Virtualizing here lets imports such as `/figures/a.typ`
        // resolve against `root` while still rejecting main files outside it.
        let vpath = VirtualPath::virtualize(root, input).map_err(|err| {
            format!(
                "Could not virtualize input path {} relative to root {}: {err}",
                input.display(),
                root.display()
            )
        })?;
        let main = RootedPath::new(VirtualRoot::Project, vpath).intern();
        // Package downloads are delegated to typst-kit so this World behaves
        // like the Typst CLI for package-backed imports.
        let downloader = SystemDownloader::new("tynding");
        let packages = SystemPackages::new(downloader);
        let files = FileStore::new(SystemFiles::new(FsRoot::new(root.to_path_buf()), packages));

        Ok(Self {
            library: LazyHash::new(library),
            book,
            fonts,
            files,
            main,
            time: Time::system(),
        })
    }
}

fn build_font_book(
    fonts: &[Font],
    font_path: Option<&str>,
    ignore_system_fonts: bool,
) -> LazyHash<FontBook> {
    // FontBook construction walks all loaded font metadata. Cache the default
    // books because they are shared across calls, but rebuild when a custom
    // `font_path` is supplied so that request-specific fonts are visible.
    if font_path.is_some() {
        return LazyHash::new(FontBook::from_fonts(fonts));
    }

    if ignore_system_fonts {
        DEFAULT_BOOK_WITHOUT_SYSTEM
            .get_or_init(|| LazyHash::new(FontBook::from_fonts(fonts)))
            .clone()
    } else {
        DEFAULT_BOOK
            .get_or_init(|| LazyHash::new(FontBook::from_fonts(fonts)))
            .clone()
    }
}

impl World for TyndingWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<typst::syntax::Source> {
        self.files.source(id)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.files.file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        self.time.today(offset)
    }
}

pub fn compile_paged_document(
    world: &TyndingWorld,
) -> std::result::Result<(PagedDocument, Vec<typst::diag::SourceDiagnostic>), CompileErrors> {
    compile_document(world)
}

pub fn compile_html_document(
    world: &TyndingWorld,
) -> std::result::Result<
    (typst_html::HtmlDocument, Vec<typst::diag::SourceDiagnostic>),
    CompileErrors,
> {
    compile_document(world)
}

type CompileErrors = typst::ecow::EcoVec<typst::diag::SourceDiagnostic>;

fn compile_document<T>(
    world: &TyndingWorld,
) -> std::result::Result<(T, Vec<typst::diag::SourceDiagnostic>), CompileErrors>
where
    T: typst::foundations::Output,
{
    let Warned { output, warnings }: Warned<SourceResult<T>> = typst::compile(world);
    // Typst memoizes compilation queries globally through comemo. The R package
    // creates short-lived worlds, so clear the cache after each compile to avoid
    // retaining sources and fonts from prior calls in a long-running R session.
    comemo::evict(0);
    output.map(|document| (document, warnings.into_iter().collect()))
}
