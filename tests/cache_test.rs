mod tests {
    use amari_rs::{api::AmariClient, defs::FetchType};
    use dotenvy::dotenv;

    use amari_rs::cache::Cache;
    use std::sync::Arc;

    #[test]
    fn test_all() {
        let mut cache = Cache::new(1000, 256 * 1024 * 1024);
        let test1: Vec<u8> = vec![1, 2, 3];

        let key = FetchType::User(111, 0);
        cache.set(&key, Arc::new(test1.clone()));

        let new_key = FetchType::User(111, 0);
        let grab1 = cache.get(&new_key).unwrap();
        let data1 = grab1.downcast_ref::<Vec<u8>>().unwrap();

        assert_eq!(data1.len(), 3);
        dbg!(&data1[0]);

        struct X {
            test: u64,
        }
        let data2 = X { test: 4423 };

        let key = FetchType::User(112, 2);
        cache.set(&key, Arc::new(data2));

        let new_key = FetchType::User(112, 2);
        let grab2 = cache.get(&new_key).unwrap();

        let data2 = grab2.downcast_ref::<X>().unwrap();
        assert_eq!(data2.test, 4423);

        dbg!(&data2.test);
    }

    #[tokio::test]
    async fn cache_bench() {
        dotenv().expect("Failed to load .env file");

        let token = std::env::var("AMARI_TOKEN").unwrap();
        let mut client = AmariClient::new(token);

        let start = std::time::Instant::now();
        let user = client.fetch_user(1087783849183940708, 607197619193643029).await;

        println!("Before cache: {}", start.elapsed().as_secs_f64());
        println!("User: {:#?}", user.unwrap().id);

        let start = std::time::Instant::now();
        let user = client.fetch_user(1087783849183940708, 607197619193643029).await;

        println!("After cache: {}", start.elapsed().as_secs_f64());
        println!("User: {:#?}", user.unwrap());

        let start = std::time::Instant::now();
        let user = client.fetch_user(1087783849183940708, 607197619193643029).await;

        println!(
            "After cache (second time): {}",
            start.elapsed().as_secs_f64()
        );
        println!("User: {:#?}", user.unwrap());
    }
}
