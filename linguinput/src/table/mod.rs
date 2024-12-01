use std::{
    collections::{hash_map, HashMap},
    sync::OnceLock,
};

use thiserror::Error;

pub mod raw;

#[derive(Debug, Clone, Copy, Error)]
pub enum TableInitError {
    #[error("Duplicated character code {} in table", .0)]
    DuplicatedCode(&'static str),
    #[error("Duplicated character {} in table", .0)]
    DuplicatedChar(&'static str),
}

#[derive(Debug)]
pub struct Table {
    max_code_len: usize,
    code_to_char: HashMap<&'static str, &'static str>,
    char_to_code: HashMap<&'static str, &'static str>,
    _priv: (),
}

impl Table {
    pub fn max_code_len(&self) -> usize {
        self.max_code_len
    }

    pub fn code_to_char(&self, input: &str) -> Option<&'static str> {
        self.code_to_char.get(input).copied()
    }

    pub fn char_to_code(&self, input: &'static str) -> Option<&'static str> {
        self.char_to_code.get(&input).copied()
    }

    pub fn load() -> Result<&'static Self, TableInitError> {
        static TABLE: OnceLock<Result<Table, TableInitError>> = OnceLock::new();
        TABLE
            .get_or_init(|| {
                let mut table = Table {
                    max_code_len: 0,
                    code_to_char: HashMap::new(),
                    char_to_code: HashMap::new(),
                    _priv: (),
                };
                for (code, ch) in raw::TABLE {
                    table.max_code_len = table.max_code_len.max(code.len());
                    match table.code_to_char.entry(code) {
                        hash_map::Entry::Occupied(_) => {
                            Err(TableInitError::DuplicatedCode(code))?
                        },
                        hash_map::Entry::Vacant(entry) => {
                            entry.insert(*ch);
                        },
                    }
                    match table.char_to_code.entry(*ch) {
                        hash_map::Entry::Occupied(_) => {
                            Err(TableInitError::DuplicatedChar(*ch))?
                        },
                        hash_map::Entry::Vacant(entry) => {
                            entry.insert(code);
                        },
                    }
                }
                Ok(table)
            })
            .as_ref()
            .map_err(|err| *err)
    }
}
