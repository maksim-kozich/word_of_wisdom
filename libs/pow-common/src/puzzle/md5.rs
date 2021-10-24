use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Md5PuzzleTask {
    pub hint: [u8; 6],
    pub hash: [u8; 16],
}

#[derive(Serialize, Deserialize)]
pub struct Md5PuzzleSolution {
    pub answer: [u8; 8],
}