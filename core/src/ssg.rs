use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Write as _,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf, StripPrefixError},
};

use tera::{Context, Tera};
use thiserror::Error;

use crate::{
    function::{invoke_fn, Function},
    markdown::page,
    Config,
};

#[derive(Debug, Error)]
pub enum InitError {
    #[error("Template directory path is not valid UTF-8")]
    TemplateDirUtf8,
    #[error("Failed to initialize Tera")]
    Tera(
        #[source]
        #[from]
        tera::Error,
    ),
}

#[derive(Debug, Error)]
#[error("Error in {}", .path.display())]
pub struct BuildError {
    path: PathBuf,
    #[source]
    kind: BuildErrorKind,
}

impl BuildError {
    fn on<E>(path: impl Into<PathBuf>) -> impl FnOnce(E) -> Self
    where
        BuildErrorKind: From<E>,
    {
        move |kind| Self { path: path.into(), kind: kind.into() }
    }
}

#[derive(Debug, Error)]
enum BuildErrorKind {
    #[error(transparent)]
    Tera(#[from] tera::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Path not compatible with UTF-8")]
    NonUtf8Path,
    #[error(transparent)]
    BadStripPrefix(#[from] StripPrefixError),
    #[error(transparent)]
    Compile(#[from] page::CompileError),
}

#[derive(Debug, Clone)]
pub struct LinSsg {
    config: Config,
    base_context: Context,
    tera: Tera,
    pages: HashMap<String, tera::Context>,
    docs: HashMap<String, String>,
}

impl LinSsg {
    const ASSET_BUF_SIZE: usize = 8192;

    pub(crate) fn new(config: Config) -> Result<Self, InitError> {
        let template_dir =
            config.template_dir().to_str().ok_or(InitError::TemplateDirUtf8)?;
        let tera = Tera::new(template_dir)?;
        Ok(Self {
            config,
            base_context: Context::new(),
            tera,
            pages: HashMap::new(),
            docs: HashMap::new(),
        })
    }

    pub fn register_const(
        &mut self,
        name: impl Into<String>,
        value: &serde_json::Value,
    ) {
        self.base_context.insert(name.into(), &value);
    }

    pub fn register_fn<F>(&mut self, name: impl Into<String>, fun: F)
    where
        F: Function,
    {
        let name = name.into();
        self.tera.register_function(
            &name.clone(),
            move |args: &HashMap<String, serde_json::Value>| match invoke_fn(
                &name, &fun, args,
            ) {
                Ok(output) => Ok(output.into()),
                Err(error) => {
                    let mut buf = format!("error in {}(", name);
                    for (i, (key, value)) in args.iter().enumerate() {
                        if i > 0 {
                            let _ = write!(buf, ", ");
                        }
                        let _ = write!(buf, "{}={}", key, value);
                    }
                    let _ = write!(buf, "):\n");
                    let mut next_source = Some(&error as &dyn Error);
                    while let Some(source) = next_source {
                        let _ = write!(buf, "- caused by: {}\n", source);
                        next_source = source.source();
                    }
                    Err(tera::Error::msg(buf))
                },
            },
        );
    }

    pub fn doc(&self, fn_name: impl AsRef<str>) -> Option<&str> {
        self.docs.get(fn_name.as_ref()).map(AsRef::as_ref)
    }

    pub fn build(&mut self) -> Result<(), BuildError> {
        self.prepare_build()?;
        self.build_pages()?;
        self.copy_assets()?;
        Ok(())
    }

    fn create_empty_output_dir(&self) -> Result<(), BuildErrorKind> {
        fs::create_dir_all(self.config.output_dir())?;
        fs::remove_dir_all(self.config.output_dir())?;
        fs::create_dir(self.config.output_dir())?;
        Ok(())
    }

    fn prepare_build(&self) -> Result<(), BuildError> {
        self.create_empty_output_dir()
            .map_err(BuildError::on(self.config.output_dir()))?;
        Ok(())
    }

    fn build_pages(&mut self) -> Result<(), BuildError> {
        self.convert_pages()?;
        self.write_pages()?;
        Ok(())
    }

    fn copy_assets(&self) -> Result<(), BuildError> {
        let mut buf = vec![0; Self::ASSET_BUF_SIZE];

        let mut directories = vec![Cow::Borrowed(self.config.asset_dir())];
        let mut expanded_symlinks = HashSet::new();
        while let Some(directory) = directories.pop() {
            let entries = fs::read_dir(directory.as_ref())
                .map_err(BuildError::on(directory.as_ref()))?;

            for result in entries {
                let entry =
                    result.map_err(BuildError::on(directory.as_ref()))?;
                let mut path = entry.path();
                let mut file_type =
                    entry.file_type().map_err(BuildError::on(&path))?;

                while file_type.is_symlink()
                    && expanded_symlinks.insert(path.clone())
                {
                    path =
                        fs::read_link(&path).map_err(BuildError::on(&path))?;
                    file_type = fs::symlink_metadata(&path)
                        .map_err(BuildError::on(&path))?
                        .file_type();
                }

                if file_type.is_dir() {
                    directories.push(Cow::Owned(path));
                } else if file_type.is_file() {
                    let mut output_path =
                        PathBuf::from(self.config.output_dir());
                    let suffix = path
                        .strip_prefix(self.config.asset_dir())
                        .map_err(BuildError::on(&path))?;
                    output_path.push("assets");
                    output_path.extend(suffix);
                    let mut output_base_dir = output_path.clone();
                    output_base_dir.pop();
                    fs::create_dir_all(&output_base_dir)
                        .map_err(BuildError::on(&output_base_dir))?;
                    let mut output_file = File::create_new(&output_path)
                        .map_err(BuildError::on(&output_path))?;
                    let mut input_file =
                        File::open(&path).map_err(BuildError::on(&path))?;

                    loop {
                        let read = input_file
                            .read(&mut buf[..])
                            .map_err(BuildError::on(&path))?;
                        if read == 0 {
                            break;
                        }
                        output_file
                            .write_all(&buf[.. read])
                            .map_err(BuildError::on(&output_path))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn convert_pages(&mut self) -> Result<(), BuildError> {
        let mut directories =
            vec![Cow::<Path>::Owned(self.config.page_dir().to_owned())];
        let mut expanded_symlinks = HashSet::new();
        while let Some(directory) = directories.pop() {
            let entries = fs::read_dir(directory.as_ref())
                .map_err(BuildError::on(directory.as_ref()))?;

            for result in entries {
                let entry =
                    result.map_err(BuildError::on(directory.as_ref()))?;
                let mut path = entry.path();
                let mut file_type =
                    entry.file_type().map_err(BuildError::on(&path))?;

                while file_type.is_symlink()
                    && expanded_symlinks.insert(path.clone())
                {
                    path =
                        fs::read_link(&path).map_err(BuildError::on(&path))?;
                    file_type = fs::symlink_metadata(&path)
                        .map_err(BuildError::on(&path))?
                        .file_type();
                }

                if file_type.is_dir() {
                    directories.push(Cow::Owned(path));
                } else if file_type.is_file() {
                    self.add_page(path)?;
                }
            }
        }

        Ok(())
    }

    fn add_page(&mut self, mut path: PathBuf) -> Result<(), BuildError> {
        let code = fs::read_to_string(&path).map_err(BuildError::on(&path))?;
        let page = page::compile(&code).map_err(BuildError::on(&path))?;

        match path.file_stem() {
            Some(stem) if !stem.eq_ignore_ascii_case("index") => {
                let directory = stem.to_owned();
                path.pop();
                path.push(directory);
                path.push("index.html");
            },
            _ => {
                path.set_extension("html");
            },
        }

        let Some(stringified_path) = path.to_str().map(ToOwned::to_owned)
        else {
            Err(BuildError { path, kind: BuildErrorKind::NonUtf8Path })?
        };
        self.tera
            .add_raw_template(&stringified_path, &page.template)
            .map_err(BuildError::on(&stringified_path))?;
        self.pages.insert(stringified_path, page.base_context);
        Ok(())
    }

    fn write_pages(&mut self) -> Result<(), BuildError> {
        for (page, context) in &self.pages {
            let mut output_page = PathBuf::from(self.config.output_dir());
            let suffix = Path::new(page)
                .strip_prefix(self.config.page_dir())
                .map_err(BuildError::on(&page))?;
            output_page.extend(suffix);
            let mut directory = output_page.clone();
            directory.pop();
            fs::create_dir_all(&directory)
                .map_err(BuildError::on(&directory))?;
            let mut output_file = File::create_new(&output_page)
                .map_err(BuildError::on(&output_page))?;
            self.tera
                .render_to(page, &context, &mut output_file)
                .map_err(BuildError::on(&output_page))?;
        }
        Ok(())
    }
}
