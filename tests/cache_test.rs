mod tests {
    use amari_rs::api::AmariClient;
    use dotenvy::dotenv;

    use amari_rs::cache::Cache;
    use std::sync::Arc;

    #[test]
    fn test_all() {
        let mut cache = Cache::new(1000, 256 * 1024 * 1024);
        let test1: Vec<u8> = vec![1, 2, 3];

        cache.set(&("test".into(), 111, 0, None), Arc::new(test1.clone()));

        let grab1 = cache.get(&("test".into(), 111, 0, None)).unwrap();
        let data1 = grab1.downcast_ref::<Vec<u8>>().unwrap();

        assert_eq!(data1.len(), 3);
        dbg!(&data1[0]);

        struct X {
            test: u64,
        }
        let data2 = X { test: 4423 };

        cache.set(&("test2".into(), 112, 2, None), Arc::new(data2));
        let grab2 = cache.get(&("test2".into(), 112, 2, None)).unwrap();

        let data2 = grab2.downcast_ref::<X>().unwrap();
        assert_eq!(data2.test, 4423);

        dbg!(&data2.test);
    }

    #[tokio::test]
    async fn cache_bench() {
        dotenv().expect("Failed to load .env file");

        let mut client = AmariClient::new();
        client.init(std::env::var("AMARI_TOKEN").unwrap());

        let start = std::time::Instant::now();
        let user = client
            .fetch_user(1087783849183940708, 607197619193643029, true)
            .await;

        println!("Before cache: {}", start.elapsed().as_secs_f64());
        println!("User: {:#?}", user.unwrap().id);

        let start = std::time::Instant::now();
        let user = client
            .fetch_user(1087783849183940708, 607197619193643029, true)
            .await;

        println!("After cache: {}", start.elapsed().as_secs_f64());
        println!("User: {:#?}", user.unwrap());

        let start = std::time::Instant::now();
        let user = client
            .fetch_user(1087783849183940708, 607197619193643029, true)
            .await;

        println!(
            "After cache (second time): {}",
            start.elapsed().as_secs_f64()
        );
        println!("User: {:#?}", user.unwrap());
    }
}
