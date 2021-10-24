use pow_client::PowClientApi;
use wow_common::{GetWisdomQuoteRequest, GetWisdomQuoteResponse};

use crate::client_api::WowClientApi;

use std::error::Error;
use pow_common::{Md5PuzzleSolution, Md5PuzzleTask};

struct ClientPow<P> {
    pow_client_api: P,
}

#[async_trait::async_trait]
impl<P> WowClientApi for ClientPow<P>
    where
        P: PowClientApi<
            Request = GetWisdomQuoteRequest,
            Response = GetWisdomQuoteResponse,
            PuzzleTask = Md5PuzzleTask,
            PuzzleSolution = Md5PuzzleSolution,
        > + Send + Sync
{
    async fn get_wisdom_quote(&self) -> Result<String, Box<dyn Error>> {
        let wow_request = GetWisdomQuoteRequest {
            temp: "temp".to_string()
        };
        let wow_response: GetWisdomQuoteResponse = self.pow_client_api.process_request(wow_request).await?;
        Ok(wow_response.quote)
    }
}