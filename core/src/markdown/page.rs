use super::to_html::{ToHtml, ToHtmlCtx, ToHtmlError};
use markdown::mdast;
use thiserror::Error;

pub const METADATA_TERMINATOR: &str = "+++";

#[derive(Debug, Error)]
#[error("{}", .message)]
pub struct MdParseError {
    message: markdown::message::Message,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    Md(#[from] MdParseError),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum SplitError {
    #[error("Missing metadata terminator line {}", METADATA_TERMINATOR)]
    MissingMetadataTerminator,
}

#[derive(Debug, Error)]
pub enum ExpandError {
    #[error(transparent)]
    ToHtml(#[from] ToHtmlError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum CompileError {
    #[error(transparent)]
    SplitError(#[from] SplitError),
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error(transparent)]
    ExpandError(#[from] ExpandError),
}

pub fn compile(code: &str) -> Result<Page, CompileError> {
    let raw_parts = RawPageParts::split(&code)?;
    let parts = raw_parts.parse()?;
    let expanded = parts.expand()?;
    Ok(expanded)
}

#[derive(Debug, Clone)]
pub struct Page {
    pub template: String,
    pub base_context: tera::Context,
}

#[derive(Debug, Clone)]
pub struct PageParts {
    pub metadata: Metadata,
    pub ast: mdast::Node,
}

impl PageParts {
    pub fn expand(&self) -> Result<Page, ExpandError> {
        let mut content = String::new();
        let mut to_html_ctx = ToHtmlCtx::default();
        self.ast.to_html(&mut content, &mut to_html_ctx)?;
        let mut context = tera::Context::new();
        context.insert("layout", &self.metadata.layout);
        context.insert("title", &self.metadata.title);
        let template = format!(
            concat!(
                "{layout_start}{layout}{layout_end}",
                "{title}",
                "{content_start}{content}{content_end}",
            ),
            layout_start = "{% extends ",
            layout = tera::to_value(&self.metadata.layout)?,
            layout_end = " %}",
            title = "{% block title %}{{ title }}{% endblock title %}",
            content_start = "{% block content %}",
            content = content,
            content_end = "{% endblock content %}",
        );
        Ok(Page { template, base_context: context })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawPageParts<'a> {
    pub metadata: &'a str,
    pub content: &'a str,
}

impl<'a> RawPageParts<'a> {
    pub fn split(code: &'a str) -> Result<Self, SplitError> {
        let mut start = 0;

        loop {
            if start >= code.len() {
                Err(SplitError::MissingMetadataTerminator)?;
            }

            let end = code[start ..]
                .find('\n')
                .map_or(code.len(), |pos| start + pos + 1);
            let line = &code[start .. end];
            if line.trim() == METADATA_TERMINATOR {
                let metadata = &code[.. start];
                let content = &code[end ..];
                break Ok(Self { metadata, content });
            }

            start = end;
        }
    }

    pub fn parse(self) -> Result<PageParts, ParseError> {
        let metadata = toml::from_str(self.metadata)?;
        let options = markdown::ParseOptions::default();
        let ast = markdown::to_mdast(self.content, &options)
            .map_err(|message| MdParseError { message })?;
        Ok(PageParts { metadata, ast })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    #[serde(default = "Metadata::default_layout")]
    layout: String,
    title: String,
}

impl Metadata {
    fn default_layout() -> String {
        String::from("default.html")
    }
}
