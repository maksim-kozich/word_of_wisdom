use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ResponseOk {
    R1,
    R2,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseErr {
    BinCode(String),
}