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
        ContentBuilder { content: String::from("") }
    }

    pub fn build_document(&self) -> &str {
        &self.content
    }

    pub fn set_document_class(&mut self, document_class: DocumentClass, options: Vec<Box<dyn ToString>>) {
        if options.is_empty() {
            self.content.push_str(&format!("\\documentclass{{{}}}\n", document_class.to_string()));
        } else {
            let options_str = options.iter().map(|o| o.to_string()).collect::<Vec<String>>().join(",");
            self.content.push_str(&format!("\\documentclass[{}]{{{}}}\n", options_str, document_class.to_string()));
        }
    }

    pub fn use_package(&mut self, package: &str, options: Vec<Box<dyn ToString>>) {
        if options.is_empty() {
            self.content.push_str(&format!("\\usepackage{{{}}}\n", package));
        } else {
            let options_str = options.iter().map(|o| o.to_string()).collect::<Vec<String>>().join(",");
            self.content.push_str(&format!("\\usepackage[{}]{{{}}}\n", options_str, package));
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
        self.content.push_str(&format!("\\title{{{}}}\n", title.merge_str()));
    }
    
    pub fn author<S: StringOrBuilder>(&mut self, author: S) {
        self.content.push_str(&format!("\\author{{{}}}\n", author.merge_str()));
    }

    pub fn maketitle(&mut self) {
        self.content.push_str("\\maketitle\n");
    }

    pub fn text_bold<S: StringOrBuilder>(&mut self, text: S) {
        self.content.push_str(&format!("\\textbf{{{}}}", text.merge_str()));
    }

    pub fn text_italic<S: StringOrBuilder>(&mut self, text: S) {
        self.content.push_str(&format!("\\textit{{{}}}", text.merge_str()));
    }

    pub fn text_underline<S: StringOrBuilder>(&mut self, text: S) {
        self.content.push_str(&format!("\\underline{{{}}}", text.merge_str()));
    }

    pub fn new_line(&mut self) {
        self.content.push_str("\\\\\n");
    }

    pub fn env<S: StringOrBuilder>(&mut self, env: Environment, content: S) {
        match env {
            Environment::Abstract
            |Environment::Center
            |Environment::Description
            |Environment::DisplayMath
            |Environment::Document
            |Environment::Enumerate
            |Environment::EqnArray
            |Environment::Equation => {
                self.content.push_str(&format!("\\begin{{{}}}\n", env.to_string()));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content.push_str(&format!("\\end{{{}}}\n", env.to_string()));
            },
            Environment::Array(params) => {
                let pos = params.pos.as_ref().map_or(String::new(), |p| format!("[{}]", p.merge_str()));
                self.content.push_str(&format!("\\begin{{{}}}{}{{{}}}\n", env.to_string(), pos, params.cols));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content.push_str(&format!("\\end{{{}}}\n", env.to_string()));
            },
            Environment::Figure(params) => {
                let placement = if params.placement.is_empty() {
                    String::new()
                } else {
                    format!("[{}]", params.placement)
                };

                self.content.push_str(&format!("\\begin{{{}}}{}\n", env.to_string(), placement));
                self.content.push_str(&format!("{}\n", content.merge_str()));
                self.content.push_str(&format!("\\end{{{}}}\n", env.to_string()));
            }
        }
    }
}