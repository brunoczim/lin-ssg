use lin_ssg_core::{Arg, ArgError, ArgParser, Args, Function};
use lin_ssg_linguinput::{
    Display,
    DisplayFormat,
    Encode,
    Encoder,
    EncodingError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranscriptionError {
    #[error("Could not encode to unicode: {}", .0)]
    Encoding(
        #[from]
        #[source]
        EncodingError,
    ),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TranscriptionType {
    GraphemicRaw,
    Graphemic,
    Morphophonemic,
    Phonemic,
    Phonetic,
}

impl<'a> Arg<'a> for TranscriptionType {
    fn from_json_ref(json: &'a serde_json::Value) -> Option<Self> {
        Some(match <&str>::from_json_ref(json)? {
            "GraphemicRaw" => Self::GraphemicRaw,
            "Morphophonemic" => Self::Morphophonemic,
            "Graphemic" => Self::Graphemic,
            "Phonemic" => Self::Phonemic,
            "Phonetic" => Self::Phonetic,
            _ => None?,
        })
    }

    fn json_type() -> String {
        "transc-type".to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TranscArgs<'a> {
    input: &'a str,
    lang: Option<&'a str>,
    ty: TranscriptionType,
    attested: bool,
}

impl<'a> Args<'a> for TranscArgs<'a> {
    fn parse(args: &mut ArgParser<'a>) -> Result<Self, ArgError> {
        let input = args.retrive_arg("in")?;
        let ty: TranscriptionType = args
            .retrive_arg_with_default("ty", || {
                TranscriptionType::GraphemicRaw
            })?;
        let lang = args.retrive_arg_with_default("lg", || None)?;
        let attested = args.retrive_arg_with_default("att", || false)?;
        Ok(Self { input, lang, ty, attested })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TranscFn;

impl Function for TranscFn {
    type Args<'a> = TranscArgs<'a>;
    type Output = String;
    type Error = TranscriptionError;

    fn call<'a>(
        &self,
        args: Self::Args<'a>,
    ) -> Result<Self::Output, Self::Error> {
        let mut buf = String::new();
        let mut encoder = Encoder::new(&mut buf)?;
        if !args.attested {
            encoder.push('*')?;
        }
        match args.ty {
            TranscriptionType::GraphemicRaw => (),
            TranscriptionType::Graphemic => {
                encoder.push_str("{<}")?;
            },
            TranscriptionType::Morphophonemic => {
                encoder.push_str("{//}")?;
            },
            TranscriptionType::Phonemic => {
                encoder.push('/')?;
            },
            TranscriptionType::Phonetic => {
                encoder.push('[')?;
            },
        }
        Display(args.input).encode(DisplayFormat, &mut encoder)?;
        match args.ty {
            TranscriptionType::GraphemicRaw => (),
            TranscriptionType::Graphemic => {
                encoder.push_str("{>}")?;
            },
            TranscriptionType::Morphophonemic => {
                encoder.push_str("{//}")?;
            },
            TranscriptionType::Phonemic => {
                encoder.push('/')?;
            },
            TranscriptionType::Phonetic => {
                encoder.push(']')?;
            },
        }
        encoder.finish()?;
        Ok(buf)
    }

    fn doc(&self) -> String {
        "{# linguistic transcriptions with unicode input #}
        transc(
            {# input #}
            in:string,
            {# language code, if not agnostic #}
            lg:string?,
            {# transcription type:
                - GraphemicRaw / GraRaw  (default)
                - Graphemic
                - Phonemic
                - Phonetic
                - Morphophonemic / Morpho
            #}
            ty:string?,
            {# attested (true) or reconstructed (false)?
                default false
            #}
            att:bool?
        ) -> String "
            .to_owned()
    }
}
