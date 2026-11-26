mod login;
mod secure_data;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    login::login().await.unwrap();
}
