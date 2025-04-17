# RustTeX

RustTeX is a Rust library for programmatically generating LaTeX documents. It provides a builder-style API to create LaTeX content with ease, allowing you to focus on the structure and content of your document without worrying about syntax errors.

## Features

- Generate LaTeX documents programmatically.
- Support for document classes, packages, sections, formatting, and more.
- Builder-style API for intuitive usage.
- Easily extendable with custom LaTeX commands.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rusttex = "0.1.2"
```

## Usage

Here are some examples of how to use RustTeX to generate LaTeX documents.

### Example 1: Basic Document

```rust
use rusttex::{ContentBuilder, DocumentClass};

let mut builder = ContentBuilder::new();
builder.set_document_class(DocumentClass::Article, options![]);
builder.use_package("amsmath", options![]);
builder.begin_document();
builder.title("My First Document");
builder.author("John Doe");
builder.maketitle();
builder.section("Introduction");
builder.add_literal("This is the introduction.");
builder.end_document();

println!("{}", builder.build_document());
```

**Generated LaTeX:**

```latex
\documentclass{article}
\usepackage{amsmath}
\begin{document}
\title{My First Document}
\author{John Doe}
\maketitle
\section{Introduction}
This is the introduction.
\end{document}
```

---

### Example 2: Formatting Text

```rust
use rusttex::ContentBuilder;

let mut builder = ContentBuilder::new();
builder.begin_document();
builder.add_literal("Here is some ");
builder.text_bold("bold text");
builder.add_literal(", ");
builder.text_italic("italic text");
builder.add_literal(", and ");
builder.text_underline("underlined text");
builder.add_literal(".");
builder.end_document();

println!("{}", builder.build_document());
```

**Generated LaTeX:**

```latex
\begin{document}
Here is some \textbf{bold text}, \textit{italic text}, and \underline{underlined text}.
\end{document}
```

---

### Example 3: Adding Sections and Subsections

```rust
use rusttex::{ContentBuilder, DocumentClass};

let mut builder = ContentBuilder::new();
builder.set_document_class(DocumentClass::Report, vec![]);
builder.begin_document();
builder.section("Main Section");
builder.subsection("Subsection 1");
builder.add_literal("Content for subsection 1.");
builder.subsection("Subsection 2");
builder.add_literal("Content for subsection 2.");
builder.end_document();

println!("{}", builder.build_document());
```

**Generated LaTeX:**

```latex
\documentclass{report}
\begin{document}
\section{Main Section}
\subsection{Subsection 1}
Content for subsection 1.
\subsection{Subsection 2}
Content for subsection 2.
\end{document}
```

---

### Example 4: Using Custom Environments

```rust
use rusttex::{ContentBuilder, Environment};

let mut builder = ContentBuilder::new();
builder.begin_document();
builder.env(Environment::Center, "This text is centered.");
builder.env(Environment::Quote, "This is a quoted block.");
builder.end_document();

println!("{}", builder.build_document());
```

**Generated LaTeX:**

```latex
\begin{document}
\begin{center}
This text is centered.
\end{center}
\begin{quote}
This is a quoted block.
\end{quote}
\end{document}
```

---

### Example 5: Adding Labels and References

```rust
use rusttex::ContentBuilder;

let mut builder = ContentBuilder::new();
builder.begin_document();
builder.section("Introduction");
builder.label("sec:intro");
builder.add_literal("See Section \\ref{sec:intro} for more details.");
builder.end_document();

println!("{}", builder.build_document());
```

**Generated LaTeX:**

```latex
\begin{document}
\section{Introduction}
\label{sec:intro}
See Section \ref{sec:intro} for more details.
\end{document}
```

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve RustTeX.
