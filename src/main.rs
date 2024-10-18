use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::{Client,header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const OPENAI_MODEL_SETTING: &str = "gpt-3.5-turbo";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

fn api_headers(key: String) -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    let header_string = format!("Bearer {}", key).parse::<String>().unwrap();
    let header_value = HeaderValue::from_str(&header_string).unwrap();
    headers.insert(AUTHORIZATION, header_value);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

#[tokio::main]
async fn main() {

    //setup env
    dotenv().ok();
    let openai_api_key = "OPENAI_API_KEY";
    let key = env::var(openai_api_key).unwrap();

    //init client
    let client = Client::new();
    let headers = api_headers(key);

    //start building request
    let mut request = OpenAIRequestBody::new(OPENAI_MODEL_SETTING);
    request.system_prompt("you are a helpful assistant");
    loop {

        //take user input
        println!("enter a message (or type exit to quit):\n");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.to_lowercase() == "exit\n" { return; }

        request.user_prompt(input.as_str());
        println!("\n---\n\nYou: {input}");
        let request_json = serde_json::to_string(&request).unwrap();
        let response = client
            .post(Url::parse(&format!("{}", OPENAI_API_URL)).unwrap())
            .headers(headers.clone())
            .body(request_json)
            .send().await.unwrap();
        let body = response.text().await.unwrap();
        let chat_completion: ChatCompletion = serde_json::from_str(&body).unwrap();
        let chat_message = &chat_completion.choices[0].message.content;
        println!("{OPENAI_MODEL_SETTING}: {chat_message}\n\n----\n");

        //remove initial user input, and complete loop
        request.messages.pop();
    }
}

// struct implementations

#[derive(Serialize, Debug)]
struct OpenAIRequestBody {
    model: String,
    messages: Vec<Message>,
}

impl OpenAIRequestBody {
    fn new(model: &str) -> Self {
        Self {
            model: String::from(model),
            messages: vec![],
        }
    }

    fn user_prompt(&mut self, message: &str) {
        self.messages.push(
            Message {
                role: "user".to_string(),
                content: message.to_string(),
            }
        );
    }

    fn system_prompt(&mut self, message: &str) {
        self.messages.push(
            Message {
                role: "system".to_string(),
                content: message.to_string(),
            }
        );
    }
}

#[derive(Serialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletion {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    pub system_fingerprint: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: i32,
    pub message: CompletionMessage,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletionMessage {
    pub role: String,
    pub content: String,
    pub refusal: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub prompt_tokens_details: PromptTokensDetails,
    pub completion_tokens_details: CompletionTokensDetails,
}

#[derive(Deserialize, Debug)]
pub struct PromptTokensDetails {
    pub cached_tokens: i32,
}

#[derive(Deserialize, Debug)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: i32,
}