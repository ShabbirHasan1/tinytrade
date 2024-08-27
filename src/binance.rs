const BASE_URL: [&str; 6] = [
    "https://api.binance.com",
    "https://api-gcp.binance.com",
    "https://api1.binance.com",
    "https://api2.binance.com",
    "https://api3.binance.com",
    "https://api4.binance.com",
];

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    last_update_id: i64,
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderTradeParams {
    symbol: String,
    side: String,
    r#type: String,
    timestamp: i64,
}

/// Refers to: https://developers.binance.com/docs/binance-spot-api-docs/rest-api#test-connectivity
pub async fn test_connectivity() -> Result<(), reqwest::Error> {
    let url = format!("{}/api/v3/ping", BASE_URL[0]);
    let _ = reqwest::get(url).await?;
    Ok(())
}

/// Refers to: https://developers.binance.com/docs/binance-spot-api-docs/rest-api#check-server-time
pub async fn check_server_time() -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/api/v3/time", BASE_URL[0]);
    let response = reqwest::get(url).await?;
    let server_time: serde_json::Value = serde_json::from_str(&response.text().await?).unwrap();
    Ok(server_time)
}

/// Refers to: https://developers.binance.com/docs/binance-spot-api-docs/rest-api#order-book
pub async fn order_book() -> Result<Orderbook, reqwest::Error> {
    let url = format!("{}/api/v3/depth", BASE_URL[0]);

    let params = [("symbol", "USDCUSDT"), ("limit", "5")];

    let response = reqwest::Client::new()
        .get(url)
        .query(&params)
        .send()
        .await?;

    let orderbook: Orderbook = serde_json::from_str(&response.text().await?).unwrap();

    Ok(orderbook)
}
