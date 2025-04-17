pub mod models;
pub mod utils;

pub use models::*;

pub trait StringOrBuilder {
    fn merge_str(self) -> String;
}

impl StringOrBuilder for &str {
    fn merge_str(self) -> String {
        self.to_string()
    }
}

pub struct ContentBuilder {
    content: String,
}

impl<F> StringOrBuilder for F
where
    F: FnOnce(&mut ContentBuilder),
{
    fn merge_str(self) -> String {
        let mut builder = ContentBuilder::new();
        self(&mut builder);
        builder.build_document().to_string()
    }
}

impl ContentBuilder {
    pub fn new() -> Self {
        ContentBuilder {
            content: String::from(""),
        }
    }

    pub fn build_document(&self) -> &str {
        &self.content
    }

    pub fn set_document_class(
        &mut self,
        document_class: DocumentClass,
        options: Vec<Box<dyn ToString>>,
    ) {
        if options.is_empty() {
            self.content.push_str(&format!(
                "\\documentclass{{{}}}\n",
                document_class.to_string()
            ));
        } else {
            let options_str = options
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join(",");
            self.content.push_str(&format!(
                "\\documentclass[{}]{{{}}}\n",
                options_str,
                document_class.to_string()
            ));
        }
    }

    pub fn use_package(&mut self, package: &str, options: Vec<Box<dyn ToString>>) {
        if options.is_empty() {
            self.content
                .push_str(&format!("\\usepackage{{{}}}\n", package));
        } else {
            let options_str = options
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join(",");
            self.content
                .push_str(&format!("\\usepackage[{}]{{{}}}\n", options_str, package));
        }
    }

    pub fn add_literal(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn begin_document(&mut self) {
        self.content.push_str("\\begin{document}\n");
    }

    pub fn end_document(&mut self) {
        self.content.push_str("\\end{document}\n");
    }

    pub fn title<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\title{{{}}}\n", title.merge_str()));
    }

    pub fn author<S: StringOrBuilder>(&mut self, author: S) {
        self.content
            .push_str(&format!("\\author{{{}}}\n", author.merge_str()));
    }

    pub fn maketitle(&mut self) {
        self.content.push_str("\\maketitle\n");
    }

    pub fn text_bold<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\textbf{{{}}}", text.merge_str()));
    }

    pub fn text_italic<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\textit{{{}}}", text.merge_str()));
    }

    pub fn text_underline<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\underline{{{}}}", text.merge_str()));
    }

    pub fn new_line(&mut self) {
        self.content.push_str("\\\\\n");
    }

    pub fn label<S: StringOrBuilder>(&mut self, label: S) {
        self.content
            .push_str(&format!("\\label{{{}}}\n", label.merge_str()));
    }

    pub fn section<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\section{{{}}}\n", title.merge_str()));
    }

    pub fn subsection<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\subsection{{{}}}\n", title.merge_str()));
    }

    pub fn subsubsection<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\subsubsection{{{}}}\n", title.merge_str()));
    }

    pub fn paragraph<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\paragraph{{{}}}\n", text.merge_str()));
    }

    pub fn subparagraph<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\subparagraph{{{}}}\n", text.merge_str()));
    }

    pub fn footnote<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\footnote{{{}}}", text.merge_str()));
    }

    pub fn cite<S: StringOrBuilder, V: StringOrBuilder>(&mut self, citation: S, subcitation: Option<V>) {
        let subcitation_str = match subcitation {
            Some(sub) => format!("[{}]", sub.merge_str()),
            None => String::new(),
        };
        self.content
            .push_str(&format!("\\cite{}{{{}}}", subcitation_str, citation.merge_str()));
    }

    pub fn ref_label<S: StringOrBuilder>(&mut self, label: S) {
        self.content
            .push_str(&format!("\\ref{{{}}}", label.merge_str()));
    }

    pub fn text_color<S: StringOrBuilder, V: StringOrBuilder>(&mut self, text: S, color: V, color_model: Option<ColorModel>) {
        let color_model_str = match color_model {
            Some(model) => format!("[{}]", model.to_string()),
            None => String::new(),
        };
        self.content.push_str(&format!(
            "\\textcolor{}{{{}}}{{{}}}",
            color_model_str,
            color.merge_str(),
            text.merge_str()
        ));
    }

    pub fn hspace<S: StringOrBuilder>(&mut self, length: S) {
        self.content.push_str(&format!("\\hspace{{{}}}", length.merge_str()));
    }

    pub fn vspace<S: StringOrBuilder>(&mut self, length: S) {
        self.content.push_str(&format!("\\vspace{{{}}}", length.merge_str()));
    }

    pub fn include<S: StringOrBuilder>(&mut self, filename: S) {
        self.content
            .push_str(&format!("\\include{{{}}}\n", filename.merge_str()));
    }

    pub fn input<S: StringOrBuilder>(&mut self, filename: S) {
        self.content
            .push_str(&format!("\\input{{{}}}\n", filename.merge_str()));
    }

    pub fn clear_page(&mut self) {
        self.content.push_str("\\clearpage\n");
    }

    pub fn new_page(&mut self) {
        self.content.push_str("\\newpage\n");
    }

    pub fn line_break(&mut self) {
        self.content.push_str("\\linebreak\n");
    }

    pub fn page_break(&mut self) {
        self.content.push_str("\\pagebreak\n");
    }

    pub fn no_indent(&mut self) {
        self.content.push_str("\\noindent\n");
    }

    pub fn centering(&mut self) {
        self.content.push_str("\\centering\n");
    }

    pub fn env<S: StringOrBuilder>(&mut self, env: Environment, content: S) {
        match env {
            Environment::Abstract
            | Environment::Center
            | Environment::Description
            | Environment::DisplayMath
            | Environment::Document
            | Environment::Enumerate
            | Environment::EqnArray
            | Environment::Equation
            | Environment::FlushLeft
            | Environment::FlushRight
            | Environment::Itemize
            | Environment::Math
            | Environment::Quotation
            | Environment::Quote
            | Environment::Tabbing
            | Environment::Theorem
            | Environment::TitlePage
            | Environment::TrivList
            | Environment::Verbatim
            | Environment::Verse => {
                self.content
                    .push_str(&format!("\\begin{{{}}}\n", env.to_string()));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::Array(params) => {
                let pos = params
                    .pos
                    .as_ref()
                    .map_or(String::new(), |p| format!("[{}]", p.merge_str()));
                self.content.push_str(&format!(
                    "\\begin{{{}}}{}{{{}}}\n",
                    env.to_string(),
                    pos,
                    params.cols
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::Figure(params) => {
                self.content.push_str(&format!(
                    "\\begin{{{}}}{}\n",
                    env.to_string(),
                    &params.placement
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::FileContents(params) => {
                let options = params
                    .option
                    .as_ref()
                    .map_or(String::new(), |o| format!("[{}]", o.to_string()));
                self.content.push_str(&format!(
                    "\\begin{{{}}}{}{{{}}}\n",
                    env.to_string(),
                    options,
                    &params.filename,
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::List(params) => {
                self.content.push_str(&format!(
                    "\\begin{{{}}}{}{}\n",
                    env.to_string(),
                    &params.labeling,
                    &params.spacing,
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::Minipage(params) => {
                let position = params
                    .position
                    .as_ref()
                    .map_or(String::from("[]"), |p| format!("[{}]", p.merge_str()));
                let height = params
                    .height
                    .as_ref()
                    .map_or(String::from("[]"), |h| format!("[{}]", h.merge_str()));
                let inner_pos = params
                    .inner_pos
                    .as_ref()
                    .map_or(String::from("[]"), |i| format!("[{}]", i.merge_str()));
                self.content.push_str(&format!(
                    "\\begin{{{}}}{}{}{}{{{}}}\n",
                    env.to_string(),
                    position,
                    height,
                    inner_pos,
                    &params.width
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::Picture(params) => {
                let size = format!("({},{})", &params.size.0, &params.size.1);
                let offset = if let Some((x, y)) = &params.offset {
                    format!("({},{})", x, y)
                } else {
                    String::new()
                };
                self.content.push_str(&format!(
                    "\\begin{{{}}}{}{}\n",
                    env.to_string(),
                    size,
                    offset
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::Table(params) => {
                let placement = params
                    .placement
                    .as_ref()
                    .map_or(String::new(), |p| format!("[{}]", p.merge_str()));
                self.content
                    .push_str(&format!("\\begin{{{}}}{}\n", env.to_string(), placement));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::Tabular(params) => {
                let pos = params
                    .pos
                    .as_ref()
                    .map_or(String::new(), |p| format!("[{}]", p.merge_str()));
                self.content.push_str(&format!(
                    "\\begin{{{}}}{}{{{}}}\n",
                    env.to_string(),
                    pos,
                    params.cols
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
            Environment::TheBibliography(params) => {
                self.content.push_str(&format!(
                    "\\begin{{{}}}{{{}}}\n",
                    env.to_string(),
                    &params.widest_label,
                ));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content
                    .push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
        }
    }
}
