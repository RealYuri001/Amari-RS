mod tests {
    use dotenvy::dotenv;
    use amari_rs::api::AmariClient;

    #[tokio::test]
    async fn cache_bench() {
        dotenv().expect("Failed to load .env file");

        let mut client = AmariClient::new();
        client.init(std::env::var("AMARI_TOKEN").unwrap());

        let start = std::time::Instant::now();
        let _ = client.fetch_user(1087783849183940708, 607197619193643029, true).await;
        
        println!("Before cache: {}", start.elapsed().as_secs_f64());

        let start = std::time::Instant::now();
        let _ = client.fetch_user(1087783849183940708, 607197619193643029, true).await;

        println!("After cache: {}", start.elapsed().as_secs_f64());

        let start = std::time::Instant::now();
        let _ = client.fetch_user(1087783849183940708, 607197619193643029, true).await;

        println!("After cache (second time): {}", start.elapsed().as_secs_f64());
    }
}