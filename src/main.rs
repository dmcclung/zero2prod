use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");
    run(listener)?.await
}
