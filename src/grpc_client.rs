use tonic::transport::Channel;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient,
    chat_service_client::ChatServiceClient,
    PaymentRequest, TransactionRequest, ChatMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PaymentService (unary)
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let response = client.process_payment(request).await?;
    println!("RESPONSE={:?}", response.into_inner());

    // TransactionService (server streaming)
    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut stream = transaction_client.get_transaction_history(request).await?.into_inner();
    while let Some(transaction) = stream.message().await? {
        println!("Transaction: {:?}", transaction);
    }

    // ChatService (bidirectional streaming)
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut chat_client = ChatServiceClient::new(channel);
    let (tx, rx) = mpsc::channel(32);

    // Spawning thread untuk baca input dari user
    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            if line.trim().is_empty() { continue; }

            let message = ChatMessage {
                user_id: "user_123".to_string(),
                message: line,
            };

            if tx.send(message).await.is_err() {
                eprintln!("Failed to send message to server.");
                break;
            }
        }
    });

    let request = tonic::Request::new(ReceiverStream::new(rx));
    let mut response_stream = chat_client.chat(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        println!("Server says: {:?}", response);
    }

    Ok(())
}