use amqprs::{
    Ack, BasicProperties, Cancel, Close, CloseChannel, Deliver, Nack, Return, 
    callbacks::{ChannelCallback, ConnectionCallback}, 
    channel::{BasicAckArguments, BasicCancelArguments, BasicConsumeArguments, Channel, QueueDeclareArguments}, 
    connection::{Connection, OpenConnectionArguments}, 
    consumer::AsyncConsumer, 
    error::Error
};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Notify;

struct ExampleConnectionCallback;

#[allow(unused_variables)]
#[async_trait]
impl ConnectionCallback for ExampleConnectionCallback {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<(), Error> {
        println!("Connection closed: {:?}", close);
        Ok(())
    }

    async fn blocked(&mut self, connection: &Connection, reason: String) {
        println!("Connection blocked: {}", reason);
    }
    
    async fn unblocked(&mut self, connection: &Connection) {
        println!("Connection unblocked");
    }
    
    async fn secret_updated(&mut self, connection: &Connection) {
        println!("Connection secret updated");
    }
}

////////////////////////////////////////////////////////////////////////////////
struct ExampleChannelCallback;

#[allow(unused_variables)]
#[async_trait]
impl ChannelCallback for ExampleChannelCallback {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<(), Error> {
        println!("Channel closed: {:?}", close);
        Ok(())
    }
    
    async fn cancel(&mut self, channel: &Channel, cancel: Cancel) -> Result<(), Error> {
        println!("Consumer cancelled: {:?}", cancel);
        Ok(())
    }
    
    async fn flow(&mut self, channel: &Channel, active: bool) -> Result<bool, Error> {
        println!("Flow control: {}", if active { "active" } else { "inactive" });
        Ok(true)
    }
    
    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {
        println!("Message acknowledged: {:?}", ack);
    }
    
    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {
        println!("Message nack'd: {:?}", nack);
    }
    
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        println!("Message returned: {:?}, content: {:?}", ret, String::from_utf8_lossy(&content));
    }
}

// Кастомный consumer для обработки сообщений
struct MessageConsumer {
    channel: Channel,
}

#[async_trait]
impl AsyncConsumer for MessageConsumer {
    async fn consume(
        &mut self,
        _channel: &Channel,
        deliver: Deliver,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        // Выводим полученное сообщение в консоль
        let message = String::from_utf8_lossy(&content);
        println!("Received message:");
        println!("  Delivery tag: {}", deliver.delivery_tag());
        println!("  Exchange: {}", deliver.exchange());
        println!("  Routing key: {}", deliver.routing_key());
        println!("  Content: {}", message);
        println!("  Properties: {:?}", basic_properties);
        println!("---");
        
        // Подтверждаем получение сообщения
        if let Err(e) = self.channel
            .basic_ack(BasicAckArguments { 
                delivery_tag: deliver.delivery_tag(), 
                multiple: false 
            })
            .await
        {
            eprintln!("Failed to ack message: {:?}", e);
        }
    }
}

async fn setup_and_consume() -> Result<(), String> {
    // Параметры подключения
    let args: OpenConnectionArguments = OpenConnectionArguments::new("192.168.0.109", 5672, "guest", "guest");
    // Открываем соединение
    let connection: Connection = Connection::open(&args).await
        .map_err(|e: Error| format!("Failed to open connection: {:?}", e))?;
    connection.register_callback(ExampleConnectionCallback)
        .await
        .map_err(|e: Error| format!("Failed to register connection callback: {:?}", e))?;
    // Открываем канал
    let channel: Channel = connection.open_channel(None).await
        .map_err(|e: Error| format!("Failed to open channel: {:?}", e))?;
    channel.register_callback(ExampleChannelCallback)
        .await
        .map_err(|e: Error| format!("Failed to register channel callback: {:?}", e))?;
    // Объявляем очередь
    let queue_args: QueueDeclareArguments = QueueDeclareArguments::default()
        .queue("my_queue".to_string()).durable(true).finish();
    let queue_info: Option<(String, u32, u32)> = channel.queue_declare(queue_args).await
        .map_err(|e: Error| format!("Failed to declare queue: {:?}", e))?;
    let queue_name: String = match queue_info {
        Some((name, _, _)) => name,
        None => return Err("Failed to get queue name".to_string()),
    };
    println!("Queue declared: {}", queue_name);
    // Создаем consumer
    let consumer: MessageConsumer = MessageConsumer {
        channel: channel.clone(),
    };
    // Начинаем слушать очередь
    let consume_args: BasicConsumeArguments = BasicConsumeArguments::new(
        &queue_name,
        "example_consumer"
    ).auto_ack(false).finish();
    println!("Starting to consume messages... Press Ctrl+C to stop.");
    // Запускаем consumer
    let _consumer_tag: String = channel.basic_consume(consumer, consume_args).await
        .map_err(|e: Error| format!("Failed to start consumer: {:?}", e))?;
    // Используем Notify для бесконечного ожидания
    let notify: Arc<Notify> = Arc::new(Notify::new());
    let notify_clone: Arc<Notify> = notify.clone();
    // Обработка Ctrl+C
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("Shutting down...");
        notify_clone.notify_one();
    });
    // Ждем сигнала завершения
    notify.notified().await;
    // Отменяем consumer
    if let Err(e) = channel.basic_cancel(BasicCancelArguments::new("example_consumer")).await {
        eprintln!("Failed to cancel consumer: {:?}", e);
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), String> {
    println!("RabbitMQ Consumer starting...");
    setup_and_consume().await?;
    println!("Consumer stopped.");
    Ok(())
}