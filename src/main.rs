mod binance;
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match binance::test_connectivity().await {
        Ok(x) => println!("{:?}", x),
        Err(e) => eprintln!("Error: {}", e),
    }

    match binance::check_server_time().await {
        Ok(x) => println!("{:?}", x),
        Err(e) => eprintln!("Error: {}", e),
    }

    match binance::order_book().await {
        Ok(x) => println!("{:?}", x),
        Err(e) => eprintln!("Error: {}", e),
    }
}
