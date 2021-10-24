use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetWisdomQuoteRequest {
    pub temp: String,
}