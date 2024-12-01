use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use serde_json::Value;
use thiserror::Error;

pub fn invoke_fn<F>(
    fn_name: &str,
    fun: &F,
    args: &HashMap<String, serde_json::Value>,
) -> Result<F::Output, InvokeError<F::Error>>
where
    F: Function,
{
    let mut arg_parser = ArgParser::new(fn_name, args);
    let parsed_args = Args::parse(&mut arg_parser).map_err(InvokeError::Arg)?;
    arg_parser.finish().map_err(InvokeError::Arg)?;
    fun.call(parsed_args).map_err(InvokeError::Execution)
}

#[derive(Debug, Error)]
pub enum InvokeError<E> {
    #[error(transparent)]
    Arg(ArgError),
    #[error(transparent)]
    Execution(E),
}

#[derive(Debug, Error)]
pub enum ArgError {
    #[error("argument {} is required but it is missing", .0)]
    MissingArgument(String),
    #[error("argument {} type should be {} but it is mismatched", .arg, .ty)]
    MismatchedTypes { arg: String, ty: String },
    #[error("argument {} is unknown", .0)]
    UnknownArguments(String),
}

pub trait Function: Send + Sync + 'static {
    type Args<'a>: Args<'a>;
    type Output: Into<serde_json::Value>;
    type Error: Error;

    fn call<'a>(
        &self,
        args: Self::Args<'a>,
    ) -> Result<Self::Output, Self::Error>;

    fn doc(&self) -> String;
}

pub trait Args<'a>: Sized {
    fn parse(arg_parser: &mut ArgParser<'a>) -> Result<Self, ArgError>;
}

pub trait Arg<'a>: Sized {
    fn from_json_ref(json: &'a Value) -> Option<Self>;

    fn json_type() -> String;
}

impl<'a> Arg<'a> for bool {
    fn from_json_ref(json: &'a Value) -> Option<Self> {
        json.as_bool()
    }

    fn json_type() -> String {
        "boolean".to_owned()
    }
}

impl<'a> Arg<'a> for i64 {
    fn from_json_ref(json: &'a Value) -> Option<Self> {
        json.as_i64()
    }

    fn json_type() -> String {
        "int64".to_owned()
    }
}

impl<'a> Arg<'a> for u64 {
    fn from_json_ref(json: &'a Value) -> Option<Self> {
        json.as_u64()
    }

    fn json_type() -> String {
        "uint64".to_owned()
    }
}

impl<'a> Arg<'a> for f64 {
    fn from_json_ref(json: &'a Value) -> Option<Self> {
        json.as_f64()
    }

    fn json_type() -> String {
        "float64".to_owned()
    }
}

impl<'a> Arg<'a> for &'a str {
    fn from_json_ref(json: &'a Value) -> Option<Self> {
        json.as_str()
    }

    fn json_type() -> String {
        "string".to_owned()
    }
}

impl<'a, A> Arg<'a> for Option<A>
where
    A: Arg<'a>,
{
    fn from_json_ref(json: &'a Value) -> Option<Self> {
        if json.is_null() {
            Some(None)
        } else {
            A::from_json_ref(json).map(Some)
        }
    }

    fn json_type() -> String {
        format!("optional {}", A::json_type())
    }
}

#[derive(Debug, Clone)]
pub struct ArgParser<'a> {
    fn_name: &'a str,
    args: &'a HashMap<String, Value>,
    unknown: HashSet<&'a str>,
}

impl<'a> ArgParser<'a> {
    pub fn new(fn_name: &'a str, args: &'a HashMap<String, Value>) -> Self {
        Self {
            fn_name,
            args,
            unknown: args.keys().map(String::as_ref).collect(),
        }
    }

    #[allow(unused)]
    pub fn fn_name(&self) -> &'a str {
        self.fn_name
    }

    pub fn retrive_arg<A>(&mut self, name: &str) -> Result<A, ArgError>
    where
        A: Arg<'a>,
    {
        let json = self
            .args
            .get(name)
            .ok_or_else(|| ArgError::MissingArgument(name.to_owned()))?;
        let arg = A::from_json_ref(json).ok_or_else(|| {
            ArgError::MismatchedTypes {
                arg: name.to_owned(),
                ty: A::json_type(),
            }
        })?;
        self.unknown.remove(name);
        Ok(arg)
    }

    pub fn retrive_arg_with_default<A, F>(
        &mut self,
        name: &str,
        default: F,
    ) -> Result<A, ArgError>
    where
        A: Arg<'a>,
        F: FnOnce() -> A,
    {
        let arg = match self.args.get(name) {
            Some(json) => A::from_json_ref(json).ok_or_else(|| {
                ArgError::MismatchedTypes {
                    arg: name.to_owned(),
                    ty: A::json_type(),
                }
            })?,
            None => default(),
        };
        self.unknown.remove(name);
        Ok(arg)
    }

    fn finish(self) -> Result<(), ArgError> {
        if self.unknown.is_empty() {
            Ok(())
        } else {
            Err(ArgError::UnknownArguments(format!(
                "function {} was not expecting arguments {} in this call",
                self.fn_name,
                self.unknown
                    .iter()
                    .fold(String::new(), |acc, elem| acc + ", " + elem),
            )))
        }
    }
}
