use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PowRequest1<Rq> {
    pub request: Rq,
}

#[derive(Serialize, Deserialize)]
pub struct PowRequest2<Pz> {
    pub puzzle_solution: Pz,
}