use std::error::Error;

#[async_trait::async_trait]
pub trait WowClientApi {
    async fn get_wisdom_quote(&self) -> Result<String, Box<dyn Error>>;
}