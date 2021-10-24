use pow_common::{PowResponseErr, PowResponse1};

#[async_trait::async_trait]
pub trait PowClient {
    type Request: ::serde::Serialize + Send;
    type Response: ::serde::de::DeserializeOwned + Send;
    type PuzzleTask: ::serde::de::DeserializeOwned + Send;
    type PuzzleSolution: ::serde::Serialize + Send;

    async fn process_request_1(&self, request: Self::Request) -> Result<PowResponse1<Self::Response, Self::PuzzleTask>, PowResponseErr>;

    async fn solve_puzzle_task(&self, puzzle_task: Self::PuzzleTask) -> Self::PuzzleSolution;

    async fn process_request_2(&self, puzzle_solution: Self::PuzzleSolution) -> Result<Self::Response, PowResponseErr>;
}