mod account_operations;
mod secure_data;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    account_operations::login().await.unwrap();
}
