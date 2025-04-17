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

/// A builder for programmatically generating LaTeX documents.
///
/// # Example
/// ```rust
/// use rusttex::{ContentBuilder, DocumentClass};
/// 
/// let mut builder = ContentBuilder::new();
/// builder.set_document_class(DocumentClass::Article, options![]);
/// builder.begin_document();
/// builder.title("Example Document");
/// builder.end_document();
/// 
/// println!("{}", builder.build_document());
/// ```
///
/// **Generated LaTeX:**
/// ```latex
/// \documentclass{article}
/// \begin{document}
/// \title{Example Document}
/// \end{document}
/// ```
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
    /// Creates a new `ContentBuilder` instance.
    ///
    /// # Example
    /// ```rust
    /// let builder = ContentBuilder::new();
    /// ```
    pub fn new() -> Self {
        ContentBuilder {
            content: String::from(""),
        }
    }

    /// Builds and returns the generated LaTeX document as a string slice.
    ///
    /// # Example
    /// ```rust
    /// let builder = ContentBuilder::new();
    /// println!("{}", builder.build_document());
    /// ```
    pub fn build_document(&self) -> &str {
        &self.content
    }

    /// Sets the document class for the LaTeX document.
    ///
    /// # Parameters
    /// - `document_class`: The document class (e.g., `DocumentClass::Article`).
    /// - `options`: A list of options for the document class.
    ///
    /// # Example
    /// ```rust
    /// use rusttex::{ContentBuilder, DocumentClass, options};
    ///
    /// let mut builder = ContentBuilder::new();
    /// builder.set_document_class(DocumentClass::Article, options!["a4paper", "twocolumn"]);
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \documentclass[a4paper,twocolumn]{article}
    /// ```
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

    /// Adds a LaTeX package to the document.
    ///
    /// # Parameters
    /// - `package`: The name of the package (e.g., `"amsmath"`).
    /// - `options`: A list of options for the package.
    ///
    /// # Example
    /// ```rust
    /// use rusttex::{ContentBuilder, options};
    ///
    /// let mut builder = ContentBuilder::new();
    /// builder.use_package("amsmath", options!["fleqn"]);
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \usepackage[fleqn]{amsmath}
    /// ```
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

    /// Adds literal text to the document.
    ///
    /// # Parameters
    /// - `text`: The text to add.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.add_literal("This is some text.");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// This is some text.
    /// ```
    pub fn add_literal(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Begins the document environment.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.begin_document();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \begin{document}
    /// ```
    pub fn begin_document(&mut self) {
        self.content.push_str("\\begin{document}\n");
    }

    /// Ends the document environment.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.end_document();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \end{document}
    /// ```
    pub fn end_document(&mut self) {
        self.content.push_str("\\end{document}\n");
    }

    /// Sets the title of the document.
    ///
    /// # Parameters
    /// - `title`: The title text.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.title("My Document");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \title{My Document}
    /// ```
    pub fn title<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\title{{{}}}\n", title.merge_str()));
    }

    /// Sets the author of the document.
    ///
    /// # Parameters
    /// - `author`: The author text.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.author("John Doe");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \author{John Doe}
    /// ```
    pub fn author<S: StringOrBuilder>(&mut self, author: S) {
        self.content
            .push_str(&format!("\\author{{{}}}\n", author.merge_str()));
    }

    /// Adds the `\maketitle` command to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.maketitle();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \maketitle
    /// ```
    pub fn maketitle(&mut self) {
        self.content.push_str("\\maketitle\n");
    }

    /// Adds bold text to the document.
    ///
    /// # Parameters
    /// - `text`: The text to make bold.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.text_bold("Bold Text");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \textbf{Bold Text}
    /// ```
    pub fn text_bold<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\textbf{{{}}}", text.merge_str()));
    }

    /// Adds italic text to the document.
    ///
    /// # Parameters
    /// - `text`: The text to italicize.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.text_italic("Italic Text");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \textit{Italic Text}
    /// ```
    pub fn text_italic<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\textit{{{}}}", text.merge_str()));
    }

    /// Adds underlined text to the document.
    ///
    /// # Parameters
    /// - `text`: The text to underline.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.text_underline("Underlined Text");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \underline{Underlined Text}
    /// ```
    pub fn text_underline<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\underline{{{}}}", text.merge_str()));
    }

    /// Adds a new line to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.new_line();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \\
    /// ```
    pub fn new_line(&mut self) {
        self.content.push_str("\\\\\n");
    }

    /// Adds a label to the document.
    ///
    /// # Parameters
    /// - `label`: The label text.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.label("sec:intro");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \label{sec:intro}
    /// ```
    pub fn label<S: StringOrBuilder>(&mut self, label: S) {
        self.content
            .push_str(&format!("\\label{{{}}}\n", label.merge_str()));
    }

    /// Adds a section to the document.
    ///
    /// # Parameters
    /// - `title`: The title of the section.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.section("Introduction");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \section{Introduction}
    /// ```
    pub fn section<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\section{{{}}}\n", title.merge_str()));
    }

    /// Adds a subsection to the document.
    ///
    /// # Parameters
    /// - `title`: The title of the subsection.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.subsection("Background");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \subsection{Background}
    /// ```
    pub fn subsection<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\subsection{{{}}}\n", title.merge_str()));
    }

    /// Adds a subsubsection to the document.
    ///
    /// # Parameters
    /// - `title`: The title of the subsubsection.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.subsubsection("Details");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \subsubsection{Details}
    /// ```
    pub fn subsubsection<S: StringOrBuilder>(&mut self, title: S) {
        self.content
            .push_str(&format!("\\subsubsection{{{}}}\n", title.merge_str()));
    }

    /// Adds a paragraph to the document.
    ///
    /// # Parameters
    /// - `text`: The text of the paragraph.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.paragraph("This is a paragraph.");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \paragraph{This is a paragraph.}
    /// ```
    pub fn paragraph<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\paragraph{{{}}}\n", text.merge_str()));
    }

    /// Adds a subparagraph to the document.
    ///
    /// # Parameters
    /// - `text`: The text of the subparagraph.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.subparagraph("This is a subparagraph.");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \subparagraph{This is a subparagraph.}
    /// ```
    pub fn subparagraph<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\subparagraph{{{}}}\n", text.merge_str()));
    }

    /// Adds a footnote to the document.
    ///
    /// # Parameters
    /// - `text`: The text of the footnote.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.footnote("This is a footnote.");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \footnote{This is a footnote.}
    /// ```
    pub fn footnote<S: StringOrBuilder>(&mut self, text: S) {
        self.content
            .push_str(&format!("\\footnote{{{}}}", text.merge_str()));
    }

    /// Adds a citation to the document.
    ///
    /// # Parameters
    /// - `citation`: The citation key.
    /// - `subcitation`: An optional subcitation.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.cite("doe2020", Some("p. 42"));
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \cite[p. 42]{doe2020}
    /// ```
    pub fn cite<S: StringOrBuilder, V: StringOrBuilder>(&mut self, citation: S, subcitation: Option<V>) {
        let subcitation_str = match subcitation {
            Some(sub) => format!("[{}]", sub.merge_str()),
            None => String::new(),
        };
        self.content
            .push_str(&format!("\\cite{}{{{}}}", subcitation_str, citation.merge_str()));
    }

    /// Adds a reference to a label in the document.
    ///
    /// # Parameters
    /// - `label`: The label to reference.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.ref_label("sec:intro");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \ref{sec:intro}
    /// ```
    pub fn ref_label<S: StringOrBuilder>(&mut self, label: S) {
        self.content
            .push_str(&format!("\\ref{{{}}}", label.merge_str()));
    }

    /// Adds colored text to the document.
    ///
    /// # Parameters
    /// - `text`: The text to color.
    /// - `color`: The color to apply.
    /// - `color_model`: An optional color model.
    ///
    /// # Example
    /// ```rust
    /// use rusttex::{ContentBuilder, ColorModel};
    ///
    /// let mut builder = ContentBuilder::new();
    /// builder.text_color("Colored Text", "red", Some(ColorModel::RGB));
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \textcolor[RGB]{red}{Colored Text}
    /// ```
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

    /// Adds horizontal space to the document.
    ///
    /// # Parameters
    /// - `length`: The length of the space.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.hspace("1cm");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \hspace{1cm}
    /// ```
    pub fn hspace<S: StringOrBuilder>(&mut self, length: S) {
        self.content.push_str(&format!("\\hspace{{{}}}", length.merge_str()));
    }

    /// Adds vertical space to the document.
    ///
    /// # Parameters
    /// - `length`: The length of the space.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.vspace("1cm");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \vspace{1cm}
    /// ```
    pub fn vspace<S: StringOrBuilder>(&mut self, length: S) {
        self.content.push_str(&format!("\\vspace{{{}}}", length.merge_str()));
    }

    /// Includes another LaTeX file in the document.
    ///
    /// # Parameters
    /// - `filename`: The name of the file to include.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.include("otherfile");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \include{otherfile}
    /// ```
    pub fn include<S: StringOrBuilder>(&mut self, filename: S) {
        self.content
            .push_str(&format!("\\include{{{}}}\n", filename.merge_str()));
    }

    /// Inputs another LaTeX file in the document.
    ///
    /// # Parameters
    /// - `filename`: The name of the file to input.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.input("otherfile");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \input{otherfile}
    /// ```
    pub fn input<S: StringOrBuilder>(&mut self, filename: S) {
        self.content
            .push_str(&format!("\\input{{{}}}\n", filename.merge_str()));
    }

    /// Adds a `\clearpage` command to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.clear_page();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \clearpage
    /// ```
    pub fn clear_page(&mut self) {
        self.content.push_str("\\clearpage\n");
    }

    /// Adds a `\newpage` command to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.new_page();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \newpage
    /// ```
    pub fn new_page(&mut self) {
        self.content.push_str("\\newpage\n");
    }

    /// Adds a `\linebreak` command to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.line_break();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \linebreak
    /// ```
    pub fn line_break(&mut self) {
        self.content.push_str("\\linebreak\n");
    }

    /// Adds a `\pagebreak` command to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.page_break();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \pagebreak
    /// ```
    pub fn page_break(&mut self) {
        self.content.push_str("\\pagebreak\n");
    }

    /// Adds a `\noindent` command to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.no_indent();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \noindent
    /// ```
    pub fn no_indent(&mut self) {
        self.content.push_str("\\noindent\n");
    }

    /// Adds a `\centering` command to the document.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.centering();
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \centering
    /// ```
    pub fn centering(&mut self) {
        self.content.push_str("\\centering\n");
    }

    /// Adds an item to an itemized list in the document.
    ///
    /// # Parameters
    /// - `content`: The content of the item.
    ///
    /// # Example
    /// ```rust
    /// let mut builder = ContentBuilder::new();
    /// builder.itemize("Item 1");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \item {Item 1}
    /// ```
    pub fn itemize<S: StringOrBuilder>(&mut self, content: S) {
        self.content
            .push_str(&format!("\\item {{{}}}\n", content.merge_str()));
    }

    /// Adds an environment to the document.
    ///
    /// # Parameters
    /// - `env`: The environment to add.
    /// - `content`: The content of the environment.
    ///
    /// # Example
    /// ```rust
    /// use rusttex::{ContentBuilder, Environment};
    ///
    /// let mut builder = ContentBuilder::new();
    /// builder.env(Environment::Abstract, "This is an abstract.");
    /// ```
    ///
    /// **Generated LaTeX:**
    /// ```latex
    /// \begin{abstract}
    /// This is an abstract.
    /// \end{abstract}
    /// ```
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
