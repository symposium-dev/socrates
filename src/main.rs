use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{info, warn};
use rand::prelude::*;

#[derive(Debug, Deserialize)]
struct ZulipWebhook {
    data: String,
    trigger: String,
    token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZulipMessage {
    content: String,
    sender_email: String,
    sender_full_name: String,
    subject: String,
    #[serde(rename = "type")]
    message_type: String,
    stream_id: Option<i64>,
    display_recipient: Option<Value>,
}

#[derive(Debug, Serialize)]
struct ZulipResponse {
    content: String,
}

impl Default for ZulipResponse {
    fn default() -> Self {
        Self {
            content: "All I know is that I know nothing.".to_string(),
        }
    }
}

/// Represents different types of AWS Lambda events that can trigger this function.
/// Uses serde(untagged) to automatically deserialize based on which fields are present.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LambdaEventType {
    /// API Gateway event - contains HTTP request data including body as JSON string
    ApiGateway {
        /// The HTTP request body, typically contains JSON-encoded webhook data as a string
        body: Option<String>,
        #[serde(rename = "httpMethod")]
        http_method: Option<String>,
        path: Option<String>,
    },
    /// Direct Lambda invocation or other event types
    DirectInvoke {
        /// Optional message field for direct invocations (not typically used for webhooks)
        message: Option<String>,
    },
}

/// Collection of famous Socrates quotes
const SOCRATES_QUOTES: &[&str] = &[
    "All I know is that I know nothing.",
    "The only true wisdom is in knowing you know nothing.",
    "An unexamined life is not worth living.",
    "There is only one good, knowledge, and one evil, ignorance.",
    "I cannot teach anybody anything. I can only make them think.",
    "Wonder is the beginning of wisdom.",
    "To find yourself, think for yourself.",
    "Be kind, for everyone you meet is fighting a hard battle.",
    "The secret of happiness, you see, is not found in seeking more, but in developing the capacity to enjoy less.",
    "True knowledge exists in knowing that you know nothing.",
    "The way to gain a good reputation is to endeavor to be what you desire to appear.",
    "He who is not contented with what he has, would not be contented with what he would like to have.",
];

fn get_random_socrates_quote() -> &'static str {
    let mut rng = rand::rng();
    SOCRATES_QUOTES.choose(&mut rng).unwrap_or(&SOCRATES_QUOTES[0])
}

fn should_respond_to_message(message: &ZulipMessage) -> bool {
    // Check if the message mentions @socrates (case insensitive)
    let content_lower = message.content.to_lowercase();
    content_lower.contains("@socrates") || content_lower.contains("@**socrates**")
}

fn create_response(message: &ZulipMessage) -> ZulipResponse {
    let quote = get_random_socrates_quote();
    let response_content = if message.content.to_lowercase().contains("hi") {
        format!("Hello, {}! {}", message.sender_full_name, quote)
    } else {
        quote.to_string()
    };
    
    ZulipResponse {
        content: response_content,
    }
}

async fn function_handler(event: LambdaEvent<LambdaEventType>) -> Result<Value, Error> {
    info!("Received event: {:?}", event.payload);
    
    // Handle different types of Lambda events using the enum
    let body = match &event.payload {
        LambdaEventType::ApiGateway { body, .. } => {
            body.as_deref().unwrap_or("")
        }
        LambdaEventType::DirectInvoke { .. } => {
            // Direct invocation or other event types
            return Ok(json!({
                "statusCode": 200,
                "body": json!({"message": "Socrates bot is running"}).to_string()
            }));
        }
    };
    
    // Parse the Zulip webhook data
    let webhook_data: Result<ZulipWebhook, _> = serde_json::from_str(body);
    let webhook = match webhook_data {
        Ok(w) => w,
        Err(e) => {
            warn!("Failed to parse webhook data: {}", e);
            return Ok(json!({
                "statusCode": 400,
                "body": json!({"error": "Invalid webhook data"}).to_string()
            }));
        }
    };
    
    // Only handle message events
    if webhook.trigger != "message" {
        info!("Ignoring non-message trigger: {}", webhook.trigger);
        return Ok(json!({
            "statusCode": 200,
            "body": json!({"message": "Event ignored"}).to_string()
        }));
    }
    
    // Parse the message data
    let message: Result<ZulipMessage, _> = serde_json::from_str(&webhook.data);
    let message = match message {
        Ok(m) => m,
        Err(e) => {
            warn!("Failed to parse message data: {}", e);
            return Ok(json!({
                "statusCode": 400,
                "body": json!({"error": "Invalid message data"}).to_string()
            }));
        }
    };
    
    info!("Received message from {}: {}", message.sender_full_name, message.content);
    
    // Check if we should respond to this message
    if !should_respond_to_message(&message) {
        info!("Message does not mention @socrates, ignoring");
        return Ok(json!({
            "statusCode": 200,
            "body": json!({"message": "Message ignored"}).to_string()
        }));
    }
    
    // Create and return response
    let response = create_response(&message);
    info!("Responding with: {}", response.content);
    
    Ok(json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": serde_json::to_string(&response)?
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    
    info!("Starting Socrates Zulip bot...");
    
    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_should_respond_to_socrates_mention() {
        let message = ZulipMessage {
            content: "@socrates hi!".to_string(),
            sender_email: "test@example.com".to_string(),
            sender_full_name: "Test User".to_string(),
            subject: "Test".to_string(),
            message_type: "stream".to_string(),
            stream_id: Some(1),
            display_recipient: None,
        };
        
        assert!(should_respond_to_message(&message));
    }
    
    #[test]
    fn test_should_respond_to_formatted_socrates_mention() {
        let message = ZulipMessage {
            content: "Hey @**Socrates** what do you think?".to_string(),
            sender_email: "test@example.com".to_string(),
            sender_full_name: "Test User".to_string(),
            subject: "Test".to_string(),
            message_type: "stream".to_string(),
            stream_id: Some(1),
            display_recipient: None,
        };
        
        assert!(should_respond_to_message(&message));
    }
    
    #[test]
    fn test_should_not_respond_to_regular_message() {
        let message = ZulipMessage {
            content: "Hello everyone!".to_string(),
            sender_email: "test@example.com".to_string(),
            sender_full_name: "Test User".to_string(),
            subject: "Test".to_string(),
            message_type: "stream".to_string(),
            stream_id: Some(1),
            display_recipient: None,
        };
        
        assert!(!should_respond_to_message(&message));
    }
    
    #[test]
    fn test_create_response_with_greeting() {
        let message = ZulipMessage {
            content: "@socrates hi!".to_string(),
            sender_email: "test@example.com".to_string(),
            sender_full_name: "Test User".to_string(),
            subject: "Test".to_string(),
            message_type: "stream".to_string(),
            stream_id: Some(1),
            display_recipient: None,
        };
        
        let response = create_response(&message);
        assert!(response.content.contains("Hello, Test User!"));
        assert!(SOCRATES_QUOTES.iter().any(|quote| response.content.contains(quote)));
    }
    
    #[test]
    fn test_create_response_without_greeting() {
        let message = ZulipMessage {
            content: "@socrates what is wisdom?".to_string(),
            sender_email: "test@example.com".to_string(),
            sender_full_name: "Test User".to_string(),
            subject: "Test".to_string(),
            message_type: "stream".to_string(),
            stream_id: Some(1),
            display_recipient: None,
        };
        
        let response = create_response(&message);
        assert!(SOCRATES_QUOTES.contains(&response.content.as_str()));
    }
    
    #[test]
    fn test_get_random_socrates_quote() {
        let quote = get_random_socrates_quote();
        assert!(SOCRATES_QUOTES.contains(&quote));
    }
}
