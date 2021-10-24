use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PowResponse1<Rs, Pz> {
    R1(Pz),
    R2(Rs),
    Err(PowResponseErr),
}

#[derive(Serialize, Deserialize)]
pub enum PowResponse2<Rs> {
    Ok(Rs),
    Err(PowResponseErr),
}

#[derive(Serialize, Deserialize, Debug, ::thiserror::Error)]
pub enum PowResponseErr {
    BadRequest(String),
    IllegalState(String),
    WrongPuzzle
}

impl Display for PowResponseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PowResponseErr::BadRequest(msg) => {format!("BadRequest {}", msg)}
            PowResponseErr::IllegalState(msg) => {format!("IllegalState {}", msg)}
            PowResponseErr::WrongPuzzle => {"WrongPuzzle".to_string()}
        };
        write!(f, "{}", s)
    }
}