use completion_request::{ChatCompletionRequest, Message, ResponseFormat};
use completion_response::ChatCompletionResponse;
use log::debug;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use super::environ_ops::{Environ, Environment, OpenAIConfig};

pub mod completion_request;
pub mod completion_response;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostChatCompletionRequest {
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

pub async fn post_chat_completion(
    messages: Vec<Message>,
    response_format: Option<ResponseFormat>,
    user_id: Option<String>,
) -> Option<String> {
    let openai_config: OpenAIConfig = Environ::init();
    debug!("response_format: {:?}", response_format);

    let max_completion_tokens = if Environment::get_env() == Environment::Dev { None } else { Some(4096) };

    let openai_request = ChatCompletionRequest {
        model: openai_config.model,
        messages,
        stream: Some(false),
        max_completion_tokens,
        user: user_id,
        response_format,
        temperature: Some(openai_config.temperature),
        top_p: None,
        n: Some(1),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
    };

    debug!("openai_request: {:?}", openai_request);

    let client = HttpClient::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_config.api_key))
        .json(&openai_request)
        .send()
        .await;

    let response = match response {
        Ok(r) => r,
        Err(e) => {
            log::error!("OpenAI request failed: {}", e);
            return None;
        }
    };

    if response.status().is_success() {
        let response_body = response.text().await.unwrap();
        let data = serde_json::from_str::<ChatCompletionResponse>(&response_body).unwrap();
        let choice = &data.choices[0];
        debug!("Finish reason: {}", choice.finish_reason);
        let content = &choice.message.content;
        Some(content.to_string())
    } else {
        log::error!("OpenAI response NOT OK: {} Details: [{}]", response.status(), response.text().await.unwrap());
        None
    }
}
