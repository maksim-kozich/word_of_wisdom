use pow_common::{PowResponseErr, PowResponse1};

use crate::PowClient;

#[async_trait::async_trait]
pub trait PowClientApi {
    type Request: ::serde::Serialize;
    type Response: ::serde::de::DeserializeOwned;

    type PuzzleTask: ::serde::de::DeserializeOwned + Send;
    type PuzzleSolution: ::serde::Serialize + Send;

    async fn process_request(&self, request: Self::Request) -> Result<Self::Response, PowResponseErr>;
}

#[async_trait::async_trait]
impl<C> PowClientApi for C where C: PowClient + Send + Sync {
    type Request = C::Request;
    type Response = C::Response;
    type PuzzleTask = C::PuzzleTask;
    type PuzzleSolution = C::PuzzleSolution;

    async fn process_request(&self, request: Self::Request) -> Result<Self::Response, PowResponseErr> {
        let response_ok = self.process_request_1(request).await?;

        match response_ok {
            PowResponse1::R1(puzzle_task) => {
                let puzzle_solution = self.solve_puzzle_task(puzzle_task).await;
                self.process_request_2(puzzle_solution).await
            }
            PowResponse1::R2(rs) => {
                println!("(pow off)");
                Ok(rs)
            }
            PowResponse1::Err(err) => {
                Err(err)
            }
        }
    }
}