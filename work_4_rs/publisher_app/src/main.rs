use amqprs::{
    BasicProperties, channel::{BasicPublishArguments, QueueDeclareArguments}, 
    connection::{Connection, OpenConnectionArguments}, error::Error
};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("RabbitMQ Publisher starting...");
    
    let args = OpenConnectionArguments::new("192.168.0.109", 5672, "guest", "guest");
    let connection = Connection::open(&args).await
        .map_err(|e: Error| format!("Failed to open connection: {:?}", e))?;
    
    let channel = connection.open_channel(None).await
        .map_err(|e: Error| format!("Failed to open channel: {:?}", e))?;
    
    // Объявляем очередь (должна совпадать с потребителем)
    let queue_name = "my_queue";
    let queue_args = QueueDeclareArguments::default()
        .queue(queue_name.to_string())
        .durable(true).finish();
    
    channel.queue_declare(queue_args).await
        .map_err(|e: Error| format!("Failed to declare queue: {:?}", e))?;
    
    println!("Starting to publish messages to queue '{}' every second...", queue_name);
    println!("Press Ctrl+C to stop.");
    
    let mut message_counter = 1;
    
    // Обработка Ctrl+C для graceful shutdown
    tokio::select! {
        _ = async {
            loop {
                let message = format!("Message #{:04} - Hello from publisher!", message_counter);
                
                // Публикуем прямо в очередь (используем пустую строку вместо exchange)
                let publish_args = BasicPublishArguments::new("", queue_name);
                
                let properties = BasicProperties::default()
                    .with_delivery_mode(2)
                    .with_content_type("text/plain")
                    .finish();
                
                match channel.basic_publish(
                    properties,
                    message.clone().into_bytes(),
                    publish_args
                ).await {
                    Ok(_) => {
                        println!("✅ Published: {}", message);
                        message_counter += 1;
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to publish message: {:?}", e);
                    }
                }
                
                time::sleep(Duration::from_secs(1)).await;
            }
        } => {}
        _ = tokio::signal::ctrl_c() => {
            println!("\nShutting down publisher...");
        }
    }
    
    println!("Publisher stopped.");
    Ok(())
}