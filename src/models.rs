use crate::StringOrBuilder;

pub enum DocumentClass {
    Article,
    Book,
    Letter,
    Report,
    Slides,
    Custom(String)
}

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
    Custom(String)
}

pub struct ArrayParams {
    pub cols: String,
    pub pos: Option<String>,
}

impl ArrayParams {
    pub fn new<S: StringOrBuilder, V: StringOrBuilder>(cols: S, pos: Option<V>) -> Self {
        ArrayParams { pos: pos.map(|p| p.merge_str()), cols: cols.merge_str() }
    }
}

pub struct FigureParams {
    pub placement: String,
}

impl FigureParams {
    pub fn new<S: StringOrBuilder>(placement: S) -> Self {
        FigureParams { placement: placement.merge_str() }
    }
}

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

pub struct FileContentsParams{
    pub filename: String,
    pub option: Option<FileContentsOption>,
}

impl FileContentsParams {
    pub fn new<S: StringOrBuilder>(filename: S, option: Option<FileContentsOption>) -> Self {
        FileContentsParams { filename: filename.merge_str(), option }
    }
}

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
}

impl<'a> ToString for Environment<'a> {
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