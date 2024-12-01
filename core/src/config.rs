use std::path::{Path, PathBuf};

use crate::{ssg::LinSsg, InitError};

#[derive(Debug, Clone)]
pub struct Config {
    template_dir: String,
    page_dir: PathBuf,
    asset_dir: PathBuf,
    output_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            template_dir: String::from("templates/**/*"),
            page_dir: PathBuf::from("pages"),
            asset_dir: PathBuf::from("assets"),
            output_dir: PathBuf::from("public"),
        }
    }
}

impl Config {
    pub fn with_templates(mut self, template_dir: impl Into<String>) -> Self {
        self.template_dir = template_dir.into();
        self.template_dir.push_str("/**/*");
        self
    }

    pub fn with_pages(mut self, page_dir: impl Into<PathBuf>) -> Self {
        self.page_dir = page_dir.into();
        self
    }

    pub fn with_assets(mut self, asset_dir: impl Into<PathBuf>) -> Self {
        self.asset_dir = asset_dir.into();
        self
    }

    pub fn with_output(mut self, output_dir: impl Into<PathBuf>) -> Self {
        self.output_dir = output_dir.into();
        self
    }

    pub fn template_dir(&self) -> &Path {
        Path::new(&self.template_dir[.. "/**/*".len()])
    }

    pub fn page_dir(&self) -> &Path {
        &self.page_dir
    }

    pub fn asset_dir(&self) -> &Path {
        &self.asset_dir
    }

    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }

    pub fn finish(self) -> Result<LinSsg, InitError> {
        LinSsg::new(self)
    }
}
