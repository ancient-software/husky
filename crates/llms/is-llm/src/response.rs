pub mod chat_completion;

use disk_cache::traits::IsLlmCacheResponse;

use crate::*;

pub trait IsLlmResponse: IsLlmCacheResponse {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmResponse {
    ChatCompletion(LlmChatCompletionResponse),
}

impl IsLlmResponse for LlmResponse {}
