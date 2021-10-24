use pow_common::PowResponseErr;
use crate::PowClientApi;

pub struct PowClientImpl {
}

#[async_trait::async_trait]
impl PowClientApi for PowClientImpl {
    type Request = ();
    type Response = ();
    type PuzzleTask = ();
    type PuzzleSolution = ();

    async fn process_request(&self, _request: Self::Request) -> Result<Self::Response, PowResponseErr> {
        todo!()
    }
}