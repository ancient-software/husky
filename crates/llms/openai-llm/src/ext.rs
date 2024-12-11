pub use openai_api_rs::v1::api::OpenAIClient;
pub use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
pub use openai_api_rs::v1::common::GPT4_O;

use crate::*;
use tokio::runtime::Runtime;

impl OaiClient {
    pub(crate) fn complete_chat_ext(&self, request: ChatCompletionRequest) -> OaiResult<String> {
        // Create a new tokio runtime
        let rt = Runtime::new().map_err(|_| OaiError::ExtChatCompletion)?;

        // Block on the async chat completion request using tokio
        let Some(ref client_ext) = self.client_ext else {
            return Err(OaiError::EnvApiKeyNotSet);
        };
        let response = rt
            .block_on(client_ext.chat_completion(request))
            .map_err(|_| OaiError::ExtChatCompletion)?;

        // Extract the message content from the first choice
        Ok(response
            .choices
            .first()
            .ok_or(OaiError::NoChoicesReturned)?
            .message
            .content
            .clone()
            .unwrap_or_default())
    }
}
