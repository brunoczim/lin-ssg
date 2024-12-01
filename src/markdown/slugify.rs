use std::fmt;

use markdown::mdast;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SlugifyError {
    #[error("formatting error")]
    Fmt(
        #[from]
        #[source]
        fmt::Error,
    ),
    #[error("slugifying markdown node {} is not supported", .0)]
    Unsupported(String),
}

pub trait Slugify {
    fn slugify(&self, buf: &mut String) -> Result<(), SlugifyError>;
}

impl Slugify for mdast::Node {
    fn slugify(&self, buf: &mut String) -> Result<(), SlugifyError> {
        match self {
            Self::Root(node) => node.slugify(buf),
            Self::Blockquote(node) => node.slugify(buf),
            Self::FootnoteDefinition(node) => node.slugify(buf),
            Self::MdxJsxFlowElement(node) => node.slugify(buf),
            Self::List(node) => node.slugify(buf),
            Self::MdxjsEsm(node) => node.slugify(buf),
            Self::Toml(node) => node.slugify(buf),
            Self::Yaml(node) => node.slugify(buf),
            Self::Break(node) => node.slugify(buf),
            Self::InlineCode(node) => node.slugify(buf),
            Self::InlineMath(node) => node.slugify(buf),
            Self::Delete(node) => node.slugify(buf),
            Self::Emphasis(node) => node.slugify(buf),
            Self::MdxTextExpression(node) => node.slugify(buf),
            Self::FootnoteReference(node) => node.slugify(buf),
            Self::Html(node) => node.slugify(buf),
            Self::Image(node) => node.slugify(buf),
            Self::ImageReference(node) => node.slugify(buf),
            Self::MdxJsxTextElement(node) => node.slugify(buf),
            Self::Link(node) => node.slugify(buf),
            Self::LinkReference(node) => node.slugify(buf),
            Self::Strong(node) => node.slugify(buf),
            Self::Text(node) => node.slugify(buf),
            Self::Code(node) => node.slugify(buf),
            Self::Math(node) => node.slugify(buf),
            Self::MdxFlowExpression(node) => node.slugify(buf),
            Self::Heading(node) => node.slugify(buf),
            Self::Table(node) => node.slugify(buf),
            Self::ThematicBreak(node) => node.slugify(buf),
            Self::TableRow(node) => node.slugify(buf),
            Self::TableCell(node) => node.slugify(buf),
            Self::ListItem(node) => node.slugify(buf),
            Self::Definition(node) => node.slugify(buf),
            Self::Paragraph(node) => node.slugify(buf),
        }
    }
}

impl<T> Slugify for [T]
where
    T: Slugify,
{
    fn slugify(&self, buf: &mut String) -> Result<(), SlugifyError> {
        for child in self {
            child.slugify(buf)?;
        }
        Ok(())
    }
}

impl Slugify for mdast::Root {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Root".to_owned()))
    }
}

impl Slugify for mdast::Blockquote {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Blockquote".to_owned()))
    }
}

impl Slugify for mdast::FootnoteDefinition {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("FootnoteDefinition".to_owned()))
    }
}

impl Slugify for mdast::MdxJsxFlowElement {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("MdxJsxFlowElement".to_owned()))
    }
}

impl Slugify for mdast::List {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("List".to_owned()))
    }
}

impl Slugify for mdast::MdxjsEsm {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("MdxjsEsm".to_owned()))
    }
}

impl Slugify for mdast::Toml {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Toml".to_owned()))
    }
}

impl Slugify for mdast::Yaml {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Yaml".to_owned()))
    }
}

impl Slugify for mdast::Break {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Break".to_owned()))
    }
}

impl Slugify for mdast::InlineCode {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("InlineCode".to_owned()))
    }
}

impl Slugify for mdast::InlineMath {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("InlineMath".to_owned()))
    }
}

impl Slugify for mdast::Delete {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Delete".to_owned()))
    }
}

impl Slugify for mdast::Emphasis {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Emphasis".to_owned()))
    }
}

impl Slugify for mdast::MdxTextExpression {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("MdxTextExpression".to_owned()))
    }
}

impl Slugify for mdast::FootnoteReference {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("FootnoteReference".to_owned()))
    }
}

impl Slugify for mdast::Html {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Html".to_owned()))
    }
}

impl Slugify for mdast::Image {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Image".to_owned()))
    }
}

impl Slugify for mdast::ImageReference {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("ImageReference".to_owned()))
    }
}

impl Slugify for mdast::MdxJsxTextElement {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("MdxJsxTextElement".to_owned()))
    }
}

impl Slugify for mdast::Link {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Link".to_owned()))
    }
}

impl Slugify for mdast::LinkReference {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("LinkReference".to_owned()))
    }
}

impl Slugify for mdast::Strong {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Strong".to_owned()))
    }
}

impl Slugify for mdast::Text {
    fn slugify(&self, buf: &mut String) -> Result<(), SlugifyError> {
        for ch in self.value.chars() {
            if ch.is_ascii_alphabetic() {
                buf.push(ch);
            } else if !buf.is_empty() {
                if ch.is_ascii_digit() || ch == '_' {
                    buf.push(ch);
                } else {
                    buf.push('-');
                }
            }
        }
        Ok(())
    }
}

impl Slugify for mdast::Code {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Code".to_owned()))
    }
}

impl Slugify for mdast::Math {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Math".to_owned()))
    }
}

impl Slugify for mdast::MdxFlowExpression {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("MdxFlowExpression".to_owned()))
    }
}

impl Slugify for mdast::Heading {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Heading".to_owned()))
    }
}

impl Slugify for mdast::Table {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Table".to_owned()))
    }
}

impl Slugify for mdast::ThematicBreak {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("ThematicBreak".to_owned()))
    }
}

impl Slugify for mdast::TableRow {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("TableRow".to_owned()))
    }
}

impl Slugify for mdast::TableCell {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("TableCell".to_owned()))
    }
}

impl Slugify for mdast::ListItem {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("ListItem".to_owned()))
    }
}

impl Slugify for mdast::Definition {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Definition".to_owned()))
    }
}

impl Slugify for mdast::Paragraph {
    fn slugify(&self, _buf: &mut String) -> Result<(), SlugifyError> {
        Err(SlugifyError::Unsupported("Paragraph".to_owned()))
    }
}
