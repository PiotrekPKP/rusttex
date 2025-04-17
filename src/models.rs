use crate::StringOrBuilder;

/// Represents the document class for a LaTeX document.
///
/// # Example
/// ```rust
/// use rusttex::DocumentClass;
///
/// let doc_class = DocumentClass::Article;
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \documentclass{article}
/// ```
pub enum DocumentClass {
    Article,
    Book,
    Letter,
    Report,
    Slides,
    Custom(String),
}

/// Represents options for LaTeX document classes.
///
/// # Example
/// ```rust
/// use rusttex::DocumentClassOptions;
///
/// let option = DocumentClassOptions::A4Paper;
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \documentclass[a4paper]{article}
/// ```
pub enum DocumentClassOptions {
    A4Paper,
    A5Paper,
    B5Paper,
    ExecutivePaper,
    LegalPaper,
    LetterPaper,
    Draft,
    Final,
    Fleqn,
    Landscape,
    Leqno,
    OpenBib,
    TitlePage,
    NotTitlePage,
    OneColumn,
    TwoColumn,
    OneSide,
    TwoSide,
    OpenRight,
    OpenAny,
    Custom(String),
}

/// Represents color models for LaTeX.
///
/// # Example
/// ```rust
/// use rusttex::ColorModel;
///
/// let color_model = ColorModel::RGB;
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \textcolor[rgb]{1,0,0}{Red Text}
/// ```
pub enum ColorModel {
    CMYK,
    Gray,
    RGB,
    RGBFull,
    Named
}

impl ToString for ColorModel {
    fn to_string(&self) -> String {
        match &self {
            ColorModel::CMYK => String::from("cmyk"),
            ColorModel::Gray => String::from("gray"),
            ColorModel::RGB => String::from("rgb"),
            ColorModel::RGBFull => String::from("RGB"),
            ColorModel::Named => String::from("named"),
        }
    }
}

/// Parameters for the LaTeX `array` environment.
///
/// # Example
/// ```rust
/// use rusttex::{ArrayParams, StringOrBuilder};
///
/// let params = ArrayParams::new("c|c", Some("t"));
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{array}[t]{c|c}
/// ...
/// \end{array}
/// ```
pub struct ArrayParams {
    pub cols: String,
    pub pos: Option<String>,
}

impl ArrayParams {
    /// Creates a new `ArrayParams` instance.
    ///
    /// # Parameters
    /// - `cols`: Column specification.
    /// - `pos`: Optional position.
    ///
    /// # Example
    /// ```rust
    /// let params = ArrayParams::new("c|c", Some("t"));
    /// ```
    pub fn new<S: StringOrBuilder, V: StringOrBuilder>(cols: S, pos: Option<V>) -> Self {
        ArrayParams {
            pos: pos.map(|p| p.merge_str()),
            cols: cols.merge_str(),
        }
    }
}

/// Parameters for the LaTeX `figure` environment.
///
/// # Example
/// ```rust
/// use rusttex::{FigureParams, StringOrBuilder};
///
/// let params = FigureParams::new("h!");
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{figure}[h!]
/// ...
/// \end{figure}
/// ```
pub struct FigureParams {
    pub placement: String,
}

impl FigureParams {
    /// Creates a new `FigureParams` instance.
    ///
    /// # Parameters
    /// - `placement`: Placement specifier.
    ///
    /// # Example
    /// ```rust
    /// let params = FigureParams::new("h!");
    /// ```
    pub fn new<S: StringOrBuilder>(placement: S) -> Self {
        FigureParams {
            placement: placement.merge_str(),
        }
    }
}

/// Represents options for the LaTeX `filecontents` environment.
///
/// # Example
/// ```rust
/// use rusttex::FileContentsOption;
///
/// let option = FileContentsOption::Force;
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{filecontents}[force]{example.txt}
/// ...
/// \end{filecontents}
/// ```
pub enum FileContentsOption {
    Force,
    Overwrite,
    NoHeader,
    NoSearch,
    Custom(String),
}

impl ToString for FileContentsOption {
    fn to_string(&self) -> String {
        match &self {
            FileContentsOption::Force => String::from("force"),
            FileContentsOption::Overwrite => String::from("overwrite"),
            FileContentsOption::NoHeader => String::from("noheader"),
            FileContentsOption::NoSearch => String::from("nosearch"),
            FileContentsOption::Custom(custom) => custom.clone(),
        }
    }
}

/// Parameters for the LaTeX `filecontents` environment.
///
/// # Example
/// ```rust
/// use rusttex::{FileContentsParams, FileContentsOption};
///
/// let params = FileContentsParams::new("example.txt", Some(FileContentsOption::Force));
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{filecontents}[force]{example.txt}
/// ...
/// \end{filecontents}
/// ```
pub struct FileContentsParams {
    pub filename: String,
    pub option: Option<FileContentsOption>,
}

impl FileContentsParams {
    /// Creates a new `FileContentsParams` instance.
    ///
    /// # Parameters
    /// - `filename`: Name of the file.
    /// - `option`: Optional filecontents option.
    ///
    /// # Example
    /// ```rust
    /// let params = FileContentsParams::new("example.txt", Some(FileContentsOption::Force));
    /// ```
    pub fn new<S: StringOrBuilder>(filename: S, option: Option<FileContentsOption>) -> Self {
        FileContentsParams {
            filename: filename.merge_str(),
            option,
        }
    }
}

/// Parameters for the LaTeX `list` environment.
///
/// # Example
/// ```rust
/// use rusttex::{ListParams, StringOrBuilder};
///
/// let params = ListParams::new("label", "spacing");
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{list}{label}{spacing}
/// ...
/// \end{list}
/// ```
pub struct ListParams {
    pub labeling: String,
    pub spacing: String,
}

impl ListParams {
    /// Creates a new `ListParams` instance.
    ///
    /// # Parameters
    /// - `labeling`: Labeling format.
    /// - `spacing`: Spacing format.
    ///
    /// # Example
    /// ```rust
    /// let params = ListParams::new("label", "spacing");
    /// ```
    pub fn new<S: StringOrBuilder, V: StringOrBuilder>(labeling: S, spacing: V) -> Self {
        ListParams {
            labeling: labeling.merge_str(),
            spacing: spacing.merge_str(),
        }
    }
}

/// Parameters for the LaTeX `minipage` environment.
///
/// # Example
/// ```rust
/// use rusttex::{MinipageParams, StringOrBuilder};
///
/// let params = MinipageParams::new(Some("c"), Some("2cm"), None, "5cm");
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{minipage}[c][2cm][]{5cm}
/// ...
/// \end{minipage}
/// ```
pub struct MinipageParams {
    pub position: Option<String>,
    pub height: Option<String>,
    pub inner_pos: Option<String>,
    pub width: String,
}

impl MinipageParams {
    /// Creates a new `MinipageParams` instance.
    ///
    /// # Parameters
    /// - `position`: Optional position.
    /// - `height`: Optional height.
    /// - `inner_pos`: Optional inner position.
    /// - `width`: Width of the minipage.
    ///
    /// # Example
    /// ```rust
    /// let params = MinipageParams::new(Some("c"), Some("2cm"), None, "5cm");
    /// ```
    pub fn new<S: StringOrBuilder, V: StringOrBuilder, T: StringOrBuilder, U: StringOrBuilder>(
        position: Option<S>,
        height: Option<V>,
        inner_pos: Option<T>,
        width: U,
    ) -> Self {
        MinipageParams {
            position: position.map(|p| p.merge_str()),
            height: height.map(|h| h.merge_str()),
            inner_pos: inner_pos.map(|i| i.merge_str()),
            width: width.merge_str(),
        }
    }
}

/// Parameters for the LaTeX `picture` environment.
///
/// # Example
/// ```rust
/// use rusttex::{PictureParams, StringOrBuilder};
///
/// let params = PictureParams::new(("10cm", "5cm"), Some(("1cm", "1cm")));
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{picture}(10cm,5cm)(1cm,1cm)
/// ...
/// \end{picture}
/// ```
pub struct PictureParams {
    pub size: (String, String),
    pub offset: Option<(String, String)>,
}

impl PictureParams {
    /// Creates a new `PictureParams` instance.
    ///
    /// # Parameters
    /// - `size`: Dimensions of the picture.
    /// - `offset`: Optional offset.
    ///
    /// # Example
    /// ```rust
    /// let params = PictureParams::new(("10cm", "5cm"), Some(("1cm", "1cm")));
    /// ```
    pub fn new<S: StringOrBuilder, V: StringOrBuilder>(
        size: (S, S),
        offset: Option<(V, V)>,
    ) -> Self {
        PictureParams {
            size: (size.0.merge_str(), size.1.merge_str()),
            offset: offset.map(|o| (o.0.merge_str(), o.1.merge_str())),
        }
    }
}

/// Parameters for the LaTeX `table` environment.
///
/// # Example
/// ```rust
/// use rusttex::{TableParams, StringOrBuilder};
///
/// let params = TableParams::new(Some("h!"));
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{table}[h!]
/// ...
/// \end{table}
/// ```
pub struct TableParams {
    pub placement: Option<String>,
}

impl TableParams {
    /// Creates a new `TableParams` instance.
    ///
    /// # Parameters
    /// - `placement`: Optional placement specifier.
    ///
    /// # Example
    /// ```rust
    /// let params = TableParams::new(Some("h!"));
    /// ```
    pub fn new<S: StringOrBuilder>(placement: Option<S>) -> Self {
        TableParams {
            placement: placement.map(|p| p.merge_str()),
        }
    }
}

/// Parameters for the LaTeX `tabular` environment.
///
/// # Example
/// ```rust
/// use rusttex::{TabularParams, StringOrBuilder};
///
/// let params = TabularParams::new("c|c", Some("t"));
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{tabular}[t]{c|c}
/// ...
/// \end{tabular}
/// ```
pub struct TabularParams {
    pub cols: String,
    pub pos: Option<String>,
}

impl TabularParams {
    /// Creates a new `TabularParams` instance.
    ///
    /// # Parameters
    /// - `cols`: Column specification.
    /// - `pos`: Optional position.
    ///
    /// # Example
    /// ```rust
    /// let params = TabularParams::new("c|c", Some("t"));
    /// ```
    pub fn new<S: StringOrBuilder, V: StringOrBuilder>(cols: S, pos: Option<V>) -> Self {
        TabularParams {
            pos: pos.map(|p| p.merge_str()),
            cols: cols.merge_str(),
        }
    }
}

/// Parameters for the LaTeX `thebibliography` environment.
///
/// # Example
/// ```rust
/// use rusttex::{TheBubliographyParams, StringOrBuilder};
///
/// let params = TheBubliographyParams::new("99");
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{thebibliography}{99}
/// ...
/// \end{thebibliography}
/// ```
pub struct TheBubliographyParams {
    pub widest_label: String,
}

impl TheBubliographyParams {
    /// Creates a new `TheBubliographyParams` instance.
    ///
    /// # Parameters
    /// - `widest_label`: The widest label in the bibliography.
    ///
    /// # Example
    /// ```rust
    /// let params = TheBubliographyParams::new("99");
    /// ```
    pub fn new<S: StringOrBuilder>(widest_label: S) -> Self {
        TheBubliographyParams {
            widest_label: widest_label.merge_str(),
        }
    }
}

/// Represents LaTeX environments.
///
/// # Example
/// ```rust
/// use rusttex::{Environment, ArrayParams};
///
/// let env = Environment::Array(&ArrayParams::new("c|c", Some("t")));
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{array}[t]{c|c}
/// ...
/// \end{array}
/// ```
pub enum Environment<'a> {
    Abstract,
    Array(&'a ArrayParams),
    Center,
    Description,
    DisplayMath,
    Document,
    Enumerate,
    EqnArray,
    Equation,
    Figure(&'a FigureParams),
    FileContents(&'a FileContentsParams),
    FlushLeft,
    FlushRight,
    Itemize,
    List(&'a ListParams),
    Math,
    Minipage(&'a MinipageParams),
    Picture(&'a PictureParams),
    Quotation,
    Quote,
    Tabbing,
    Table(&'a TableParams),
    Tabular(&'a TabularParams),
    TheBibliography(&'a TheBubliographyParams),
    Theorem,
    TitlePage,
    TrivList,
    Verbatim,
    Verse,
}

impl<'a> ToString for Environment<'a> {
    /// Converts the environment to its LaTeX string representation.
    ///
    /// # Example
    /// ```rust
    /// use rusttex::{Environment, ArrayParams};
    ///
    /// let env = Environment::Array(&ArrayParams::new("c|c", Some("t")), "...");
    /// println!("{}", env.to_string());
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \begin{array}[t]{c|c}
    /// ...
    /// \end{array}
    /// ```
    fn to_string(&self) -> String {
        match &self {
            Environment::Abstract => String::from("abstract"),
            Environment::Array(_) => String::from("array"),
            Environment::Center => String::from("center"),
            Environment::Description => String::from("description"),
            Environment::DisplayMath => String::from("displaymath"),
            Environment::Document => String::from("document"),
            Environment::Enumerate => String::from("enumerate"),
            Environment::EqnArray => String::from("eqnarray"),
            Environment::Equation => String::from("equation"),
            Environment::Figure(_) => String::from("figure"),
            Environment::FileContents(_) => String::from("filecontents"),
            Environment::FlushLeft => String::from("flushleft"),
            Environment::FlushRight => String::from("flushright"),
            Environment::Itemize => String::from("itemize"),
            Environment::List(_) => String::from("list"),
            Environment::Math => String::from("math"),
            Environment::Minipage(_) => String::from("minipage"),
            Environment::Picture(_) => String::from("picture"),
            Environment::Quotation => String::from("quotation"),
            Environment::Quote => String::from("quote"),
            Environment::Tabbing => String::from("tabbing"),
            Environment::Table(_) => String::from("table"),
            Environment::Tabular(_) => String::from("tabular"),
            Environment::TheBibliography(_) => String::from("thebibliography"),
            Environment::Theorem => String::from("theorem"),
            Environment::TitlePage => String::from("titlepage"),
            Environment::TrivList => String::from("trivlist"),
            Environment::Verbatim => String::from("verbatim"),
            Environment::Verse => String::from("verse"),
        }
    }
}

impl ToString for DocumentClass {
    fn to_string(&self) -> String {
        match &self {
            DocumentClass::Article => String::from("article"),
            DocumentClass::Book => String::from("book"),
            DocumentClass::Letter => String::from("letter"),
            DocumentClass::Report => String::from("report"),
            DocumentClass::Slides => String::from("slides"),
            DocumentClass::Custom(custom) => custom.clone(),
        }
    }
}

impl ToString for DocumentClassOptions {
    fn to_string(&self) -> String {
        match &self {
            DocumentClassOptions::A4Paper => String::from("a4paper"),
            DocumentClassOptions::A5Paper => String::from("a5paper"),
            DocumentClassOptions::B5Paper => String::from("b5paper"),
            DocumentClassOptions::ExecutivePaper => String::from("executivepaper"),
            DocumentClassOptions::LegalPaper => String::from("legalpaper"),
            DocumentClassOptions::LetterPaper => String::from("letterpaper"),
            DocumentClassOptions::Draft => String::from("draft"),
            DocumentClassOptions::Final => String::from("final"),
            DocumentClassOptions::Fleqn => String::from("fleqn"),
            DocumentClassOptions::Landscape => String::from("landscape"),
            DocumentClassOptions::Leqno => String::from("leqno"),
            DocumentClassOptions::OpenBib => String::from("openbib"),
            DocumentClassOptions::TitlePage => String::from("titlepage"),
            DocumentClassOptions::NotTitlePage => String::from("notitlepage"),
            DocumentClassOptions::OneColumn => String::from("onecolumn"),
            DocumentClassOptions::TwoColumn => String::from("twocolumn"),
            DocumentClassOptions::OneSide => String::from("oneside"),
            DocumentClassOptions::TwoSide => String::from("twoside"),
            DocumentClassOptions::OpenRight => String::from("openright"),
            DocumentClassOptions::OpenAny => String::from("openany"),
            DocumentClassOptions::Custom(custom) => custom.clone(),
        }
    }
}
