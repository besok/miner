use sv_parser::Error;
use crate::cfg::CFGError;

mod parser;
mod cfg;

#[derive(Debug)]
pub enum MinerError {
    FileError(std::io::Error),
    CfgError(CFGError),
    ParseError(Error),
}

impl From<CFGError> for MinerError {
    fn from(value: CFGError) -> Self {
        MinerError::CfgError(value)
    }
}


impl From<std::io::Error> for MinerError {
    fn from(value: std::io::Error) -> Self {
        MinerError::FileError(value)
    }
}

impl From<Error> for MinerError {
    fn from(value: Error) -> Self {
        MinerError::ParseError(value)
    }
}