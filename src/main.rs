mod login;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    login::login().await.unwrap();
}
