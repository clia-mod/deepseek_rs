use deepseek_rs::{
    client::chat_completions::request::{Message, Model, RequestBody, Temperature},
    DeepSeekClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize the client
    let client = DeepSeekClient::default()?;

    // Basic chat completion
    let request = RequestBody::new_messages(vec![Message::new_user_message(
        "Explain what is rust programming language in one sentence.".to_string(),
    )]);
    let response = client.chat_completions(request).await?;
    println!(
        "Basic response: {}",
        response.choices[0].message.content.as_ref().unwrap()
    );

    // Chat completion with reasoning model
    let request = RequestBody::new_messages(vec![Message::new_user_message(
        "What is 15 * 7?".to_string(),
    )])
    .with_model(Model::DeepSeekReasoner);
    let response = client.chat_completions(request).await?;
    println!("\nReasoning response:");
    println!(
        "Reasoning: {}",
        response.choices[0]
            .message
            .reasoning_content
            .as_ref()
            .unwrap()
    );
    println!(
        "Final answer: {}",
        response.choices[0].message.content.as_ref().unwrap()
    );

    // Chat completion with system message and temperature
    let request = RequestBody::new_messages(vec![
        Message::new_system_message("You are a creative storyteller.".to_string()),
        Message::new_user_message("Tell me a very short story about a robot.".to_string()),
    ])
    .with_temperature(Temperature::new(0.9));
    let response = client.chat_completions(request).await?;
    println!(
        "\nCreative story: {}",
        response.choices[0].message.content.as_ref().unwrap()
    );

    Ok(())
}
