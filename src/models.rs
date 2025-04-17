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
    /// Represents the `article` document class in LaTeX.
    Article,
    /// Represents the `book` document class in LaTeX.
    Book,
    /// Represents the `letter` document class in LaTeX.
    Letter,
    /// Represents the `report` document class in LaTeX.
    Report,
    /// Represents the `slides` document class in LaTeX.
    Slides,
    /// Represents the custom document class in LaTeX.
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
    /// Represents the `a4paper` option for document classes in LaTeX.
    A4Paper,
    /// Represents the `a5paper` option for document classes in LaTeX.
    A5Paper,
    /// Represents the `b5paper` option for document classes in LaTeX.
    B5Paper,
    /// Represents the `executivepaper` option for document classes in LaTeX.
    ExecutivePaper,
    /// Represents the `legalpaper` option for document classes in LaTeX.
    LegalPaper,
    /// Represents the `letterpaper` option for document classes in LaTeX.
    LetterPaper,
    /// Represents the `draft` option for document classes in LaTeX.
    Draft,
    /// Represents the `final` option for document classes in LaTeX.
    Final,
    /// Represents the `fleqn` option for document classes in LaTeX.
    Fleqn,
    /// Represents the `landscape` option for document classes in LaTeX.
    Landscape,
    /// Represents the `leqno` option for document classes in LaTeX.
    Leqno,
    /// Represents the `openbib` option for document classes in LaTeX.
    OpenBib,
    /// Represents the `titlepage` option for document classes in LaTeX.
    TitlePage,
    /// Represents the `nottitlepage` option for document classes in LaTeX.
    NotTitlePage,
    /// Represents the `onecolumn` option for document classes in LaTeX.
    OneColumn,
    /// Represents the `twocolumn` option for document classes in LaTeX.
    TwoColumn,
    /// Represents the `oneside` option for document classes in LaTeX.
    OneSide,
    /// Represents the `twoside` option for document classes in LaTeX.
    TwoSide,
    /// Represents the `openright` option for document classes in LaTeX.
    OpenRight,
    /// Represents the `openany` option for document classes in LaTeX.
    OpenAny,
    /// Represents a custom option for document classes in LaTeX.
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
    /// Represents the `cmyk` color model in LaTeX.
    CMYK,
    /// Represents the `gray` color model in LaTeX.
    Gray,
    /// Represents the `rgb` color model in LaTeX.
    RGB,
    /// Represents the `RGB` color model in LaTeX.
    RGBFull,
    /// Represents the `named` color model in LaTeX.
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
    /// Describes the number of columns, their alignment, and the formatting of the intercolumn regions.
    pub cols: String,
    /// Specifies the table’s vertical position
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
    /// The possible values of placement are h for ‘here’, t for ‘top’, b for ‘bottom’, and p for ‘on a separate page of floats’.
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
    /// Represents the `force` option for the `filecontents` environment in LaTeX.
    Force,
    /// Represents the `overwrite` option for the `filecontents` environment in LaTeX.
    Overwrite,
    /// Represents the `noheader` option for the `filecontents` environment in LaTeX.
    NoHeader,
    /// Represents the `nosearch` option for the `filecontents` environment in LaTeX.
    NoSearch,
    /// Represents a custom option for the `filecontents` environment in LaTeX.
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
    /// The name of the file to be created or written to.
    pub filename: String,
    /// Specifies an optional filecontents option.
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
    /// Specifies the default labeling of list items.
    pub labeling: String,
    /// Specifies a list of commands.
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
    /// Governs how the minipage vertically aligns with the surrounding material.
    pub position: Option<String>,
    /// It sets the height of the minipage
    pub height: Option<String>,
    /// Specifies the inner position of the minipage.
    pub inner_pos: Option<String>,
    /// It gives the width of the box into which contents are typeset.
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
    /// Specifies the size of the picture.
    pub size: (String, String),
    /// Specifies the offset of the picture.
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
    /// Specifies the placement of the table.
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
    /// Specifies the number of columns, their alignment, and the formatting of the intercolumn regions.
    pub cols: String,
    /// Specifies the table’s vertical position.
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
    /// Specifies the widest label in the bibliography.
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
/// let env = Environment::Array(&ArrayParams::new("c|c", Some("t")), "...");
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \begin{array}[t]{c|c}
/// ...
/// \end{array}
/// ```
pub enum Environment<'a> {
    /// Represents the `abstract` environment in LaTeX.
    Abstract,
    /// Represents the `array` environment in LaTeX.
    Array(&'a ArrayParams),
    /// Represents the `center` environment in LaTeX.
    Center,
    /// Represents the `description` environment in LaTeX.
    Description,
    /// Represents the `displaymath` environment in LaTeX.
    DisplayMath,
    /// Represents the `document` environment in LaTeX.
    Document,
    /// Represents the `enumerate` environment in LaTeX.
    Enumerate,
    /// Represents the `eqnarray` environment in LaTeX.
    EqnArray,
    /// Represents the `equation` environment in LaTeX.
    Equation,
    /// Represents the `figure` environment in LaTeX.
    Figure(&'a FigureParams),
    /// Represents the `filecontents` environment in LaTeX.
    FileContents(&'a FileContentsParams),
    /// Represents the `flushleft` environment in LaTeX.
    FlushLeft,
    /// Represents the `flushright` environment in LaTeX.
    FlushRight,
    /// Represents the `itemize` environment in LaTeX.
    Itemize,
    /// Represents the `list` environment in LaTeX.
    List(&'a ListParams),
    /// Represents the `math` environment in LaTeX.
    Math,
    /// Represents the `minipage` environment in LaTeX.
    Minipage(&'a MinipageParams),
    /// Represents the `picture` environment in LaTeX.
    Picture(&'a PictureParams),
    /// Represents the `quotation` environment in LaTeX.
    Quotation,
    /// Represents the `quote` environment in LaTeX.
    Quote,
    /// Represents the `tabbing` environment in LaTeX.
    Tabbing,
    /// Represents the `table` environment in LaTeX.
    Table(&'a TableParams),
    /// Represents the `tabular` environment in LaTeX.
    Tabular(&'a TabularParams),
    /// Represents the `thebibliography` environment in LaTeX.
    TheBibliography(&'a TheBubliographyParams),
    /// Represents the `theorem` environment in LaTeX.
    Theorem,
    /// Represents the `titlepage` environment in LaTeX.
    TitlePage,
    /// Represents the `trivlist` environment in LaTeX.
    TrivList,
    /// Represents the `verbatim` environment in LaTeX.
    Verbatim,
    /// Represents the `verse` environment in LaTeX.
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
