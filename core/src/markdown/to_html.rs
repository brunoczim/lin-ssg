use std::{
    collections::HashMap,
    fmt::{self, Write as _},
};

use markdown::mdast;
use thiserror::Error;

use super::slugify::{Slugify, SlugifyError};

pub const TEMPLATE_BLOCK_START: &str = "{{";
pub const TEMPLATE_BLOCK_END: &str = "}}";

#[derive(Debug, Error)]
pub enum ToHtmlError {
    #[error("Formatting error")]
    Fmt(
        #[from]
        #[source]
        fmt::Error,
    ),
    #[error(transparent)]
    Slugify(#[from] SlugifyError),
    #[error("Converting markdown node {} to HTML is not supported", .0)]
    Unsupported(String),
    #[error("HTML/Markdown template block not closed, near {}", .0)]
    UnclosedBlock(String),
}

#[derive(Debug, Clone)]
pub struct ToHtmlCtx {
    slugs: HashMap<String, usize>,
    sections: Vec<String>,
    ord_list_depth: usize,
    unord_list_depth: usize,
}

impl ToHtmlCtx {
    #[expect(dead_code)]
    pub fn ord_list_depth(&self) -> usize {
        self.ord_list_depth
    }

    #[expect(dead_code)]
    pub fn unord_list_depth(&self) -> usize {
        self.ord_list_depth
    }

    #[expect(dead_code)]
    pub fn section_depth(&self) -> usize {
        self.sections.len()
    }

    pub fn enter_ord_list(&mut self) -> usize {
        let depth = self.ord_list_depth;
        self.ord_list_depth += 1;
        depth
    }

    pub fn enter_unord_list(&mut self) -> usize {
        let depth = self.unord_list_depth;
        self.unord_list_depth += 1;
        depth
    }

    pub fn enter_section(
        &mut self,
        depth: u8,
        title_slug: String,
        buf: &mut String,
    ) -> Result<String, ToHtmlError> {
        self.prepare_section_level(depth, buf)?;
        self.sections.push(title_slug);
        Ok(self.make_slug())
    }

    pub fn leave_ord_list(&mut self) {
        self.ord_list_depth = self.ord_list_depth.saturating_sub(1);
    }

    pub fn leave_unord_list(&mut self) {
        self.unord_list_depth = self.unord_list_depth.saturating_sub(1);
    }

    pub fn leave_section(
        &mut self,
        depth: u8,
        buf: &mut String,
    ) -> Result<(), ToHtmlError> {
        self.prepare_section_level(depth, buf)?;
        Ok(())
    }

    fn make_slug(&mut self) -> String {
        let base_slug = self.sections.join("-").to_ascii_lowercase();
        let count = self.slugs.entry(base_slug.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            format!("{}-{}", base_slug, *count)
        } else {
            base_slug
        }
    }

    fn prepare_section_level(
        &mut self,
        new_depth: u8,
        buf: &mut String,
    ) -> Result<(), ToHtmlError> {
        if let Some(close_count) =
            self.sections.len().checked_sub(usize::from(new_depth))
        {
            for _ in 0 ..= close_count {
                self.sections.pop();
                write!(buf, "</div>")?;
            }
        }
        Ok(())
    }
}

impl Default for ToHtmlCtx {
    fn default() -> Self {
        Self {
            slugs: HashMap::new(),
            sections: Vec::new(),
            ord_list_depth: 0,
            unord_list_depth: 0,
        }
    }
}

pub trait ToHtml {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError>;
}

impl ToHtml for mdast::Node {
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        match self {
            Self::Root(node) => node.to_html(buf, context),
            Self::Blockquote(node) => node.to_html(buf, context),
            Self::FootnoteDefinition(node) => node.to_html(buf, context),
            Self::MdxJsxFlowElement(node) => node.to_html(buf, context),
            Self::List(node) => node.to_html(buf, context),
            Self::MdxjsEsm(node) => node.to_html(buf, context),
            Self::Toml(node) => node.to_html(buf, context),
            Self::Yaml(node) => node.to_html(buf, context),
            Self::Break(node) => node.to_html(buf, context),
            Self::InlineCode(node) => node.to_html(buf, context),
            Self::InlineMath(node) => node.to_html(buf, context),
            Self::Delete(node) => node.to_html(buf, context),
            Self::Emphasis(node) => node.to_html(buf, context),
            Self::MdxTextExpression(node) => node.to_html(buf, context),
            Self::FootnoteReference(node) => node.to_html(buf, context),
            Self::Html(node) => node.to_html(buf, context),
            Self::Image(node) => node.to_html(buf, context),
            Self::ImageReference(node) => node.to_html(buf, context),
            Self::MdxJsxTextElement(node) => node.to_html(buf, context),
            Self::Link(node) => node.to_html(buf, context),
            Self::LinkReference(node) => node.to_html(buf, context),
            Self::Strong(node) => node.to_html(buf, context),
            Self::Text(node) => node.to_html(buf, context),
            Self::Code(node) => node.to_html(buf, context),
            Self::Math(node) => node.to_html(buf, context),
            Self::MdxFlowExpression(node) => node.to_html(buf, context),
            Self::Heading(node) => node.to_html(buf, context),
            Self::Table(node) => node.to_html(buf, context),
            Self::ThematicBreak(node) => node.to_html(buf, context),
            Self::TableRow(node) => node.to_html(buf, context),
            Self::TableCell(node) => node.to_html(buf, context),
            Self::ListItem(node) => node.to_html(buf, context),
            Self::Definition(node) => node.to_html(buf, context),
            Self::Paragraph(node) => node.to_html(buf, context),
        }
    }
}

impl<T> ToHtml for [T]
where
    T: ToHtml,
{
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        for child in self {
            child.to_html(buf, context)?;
        }
        Ok(())
    }
}

impl ToHtml for mdast::Root {
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        self.children.to_html(buf, context)?;
        context.leave_section(1, buf)?;
        Ok(())
    }
}

impl ToHtml for mdast::Blockquote {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Blockquote".to_owned()))
    }
}

impl ToHtml for mdast::FootnoteDefinition {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("FootnoteDefinition".to_owned()))
    }
}

impl ToHtml for mdast::MdxJsxFlowElement {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("MdxJsxFlowElement".to_owned()))
    }
}

impl ToHtml for mdast::List {
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        if self.ordered {
            let depth = context.enter_ord_list();
            let classes = ["arabic", "latin", "roman"];
            let class = classes[depth % classes.len()];
            write!(buf, "<ol class=\"list-{class}\"")?;
            if let Some(start) = self.start {
                write!(buf, " start=\"{start}\"")?;
            }
            write!(buf, ">")?;
            self.children.to_html(buf, context)?;
            write!(buf, "</ol>")?;
            context.leave_ord_list();
        } else {
            let depth = context.enter_unord_list();
            let classes = ["disc", "square", "circle"];
            let class = classes[depth % classes.len()];
            write!(buf, "<ul class=\"list-{class}\">")?;
            self.children.to_html(buf, context)?;
            write!(buf, "</ul>")?;
            context.leave_unord_list();
        }
        Ok(())
    }
}

impl ToHtml for mdast::MdxjsEsm {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("MdxjsEsm".to_owned()))
    }
}

impl ToHtml for mdast::Toml {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Toml".to_owned()))
    }
}

impl ToHtml for mdast::Yaml {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Yaml".to_owned()))
    }
}

impl ToHtml for mdast::Break {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Break".to_owned()))
    }
}

impl ToHtml for mdast::InlineCode {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("InlineCode".to_owned()))
    }
}

impl ToHtml for mdast::InlineMath {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("InlineMath".to_owned()))
    }
}

impl ToHtml for mdast::Delete {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Delete".to_owned()))
    }
}

impl ToHtml for mdast::Emphasis {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Emphasis".to_owned()))
    }
}

impl ToHtml for mdast::MdxTextExpression {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("MdxTextExpression".to_owned()))
    }
}

impl ToHtml for mdast::FootnoteReference {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("FootnoteReference".to_owned()))
    }
}

impl ToHtml for mdast::Html {
    fn to_html(
        &self,
        buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        write!(buf, "{}", self.value)?;
        Ok(())
    }
}

impl ToHtml for mdast::Image {
    fn to_html(
        &self,
        buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        let escaped_src = tera::escape_html(&self.url);
        let escaped_alt = tera::escape_html(&self.alt);
        write!(
            buf,
            "<div class=\"img-wrapper\"><img src=\"{}\" alt=\"{}\"/><div \
             class=\"img-legend\">{}</div></div>",
            escaped_src, escaped_alt, escaped_alt,
        )?;
        Ok(())
    }
}

impl ToHtml for mdast::ImageReference {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("ImageReference".to_owned()))
    }
}

impl ToHtml for mdast::MdxJsxTextElement {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("MdxJsxTextElement".to_owned()))
    }
}

impl ToHtml for mdast::Link {
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        write!(buf, "<a href=\"{}\"", self.url)?;
        if let Some(title) = &self.title {
            write!(buf, " title=\"{title}\"")?;
        }
        write!(buf, ">")?;
        self.children.to_html(buf, context)?;
        write!(buf, "</a>")?;
        Ok(())
    }
}

impl ToHtml for mdast::LinkReference {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("LinkReference".to_owned()))
    }
}

impl ToHtml for mdast::Strong {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Strong".to_owned()))
    }
}

impl ToHtml for mdast::Text {
    fn to_html(
        &self,
        buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum ExpandState {
            BlockRoot,
            StringLiteral,
            Escaping,
        }

        let mut value = &self.value[..];

        loop {
            let Some(expand_start) = value.find(TEMPLATE_BLOCK_START) else {
                write!(buf, "{}", tera::escape_html(value))?;
                break;
            };
            write!(buf, "{}", tera::escape_html(&value[.. expand_start]))?;
            let expanding = &value[expand_start ..];
            let mut len = TEMPLATE_BLOCK_START.len();
            let mut state = ExpandState::BlockRoot;

            loop {
                let Some(ch) = expanding[len ..].chars().next() else {
                    Err(ToHtmlError::UnclosedBlock(value.to_owned()))?
                };

                match state {
                    ExpandState::BlockRoot => {
                        if expanding[len ..].starts_with(TEMPLATE_BLOCK_END) {
                            len += TEMPLATE_BLOCK_END.len();
                            break;
                        }
                        if ch == '"' {
                            state = ExpandState::StringLiteral;
                        }
                    },
                    ExpandState::StringLiteral => {
                        if ch == '"' {
                            state = ExpandState::BlockRoot;
                        } else if ch == '\\' {
                            state = ExpandState::Escaping;
                        }
                    },
                    ExpandState::Escaping => {
                        state = ExpandState::StringLiteral;
                    },
                }
                len += ch.len_utf8();
            }
            write!(buf, "{}", &expanding[.. len])?;
            value = &expanding[len ..];
        }

        Ok(())
    }
}

impl ToHtml for mdast::Code {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Code".to_owned()))
    }
}

impl ToHtml for mdast::Math {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Math".to_owned()))
    }
}

impl ToHtml for mdast::MdxFlowExpression {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("MdxFlowExpression".to_owned()))
    }
}

impl ToHtml for mdast::Heading {
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        let depth = match self.depth {
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            _ => "6",
        };
        let id = buf.len();
        let mut title_slug = String::new();
        self.children.slugify(&mut title_slug)?;
        let full_slug = context.enter_section(self.depth, title_slug, buf)?;
        write!(
            buf,
            "<h{depth} id=\"section_{id}\"><a href=\"#section_{full_slug}\">"
        )?;
        self.children.to_html(buf, context)?;
        write!(buf, "</a></h{depth}>")?;
        write!(buf, "<div class=\"section-body\">")?;
        Ok(())
    }
}

impl ToHtml for mdast::Table {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Table".to_owned()))
    }
}

impl ToHtml for mdast::ThematicBreak {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("ThematicBreak".to_owned()))
    }
}

impl ToHtml for mdast::TableRow {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("TableRow".to_owned()))
    }
}

impl ToHtml for mdast::TableCell {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("TableCell".to_owned()))
    }
}

impl ToHtml for mdast::ListItem {
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        if self.checked.is_some() {
            Err(ToHtmlError::Unsupported("checkable ListItem".to_owned()))?;
        }
        write!(buf, "<li>")?;
        self.children.to_html(buf, context)?;
        write!(buf, "</li>")?;
        Ok(())
    }
}

impl ToHtml for mdast::Definition {
    fn to_html(
        &self,
        _buf: &mut String,
        _context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        Err(ToHtmlError::Unsupported("Definition".to_owned()))
    }
}

impl ToHtml for mdast::Paragraph {
    fn to_html(
        &self,
        buf: &mut String,
        context: &mut ToHtmlCtx,
    ) -> Result<(), ToHtmlError> {
        write!(buf, "<p>")?;
        for child in &self.children {
            child.to_html(buf, context)?;
        }
        write!(buf, "</p>")?;
        Ok(())
    }
}
