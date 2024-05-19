mod tests {
    use amari_rs::api::AmariClient;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_client() {
        dotenv().expect("Failed to load .env file");
        let token: String = std::env::var("API_SECRET").unwrap();

        let mut client = AmariClient::default();
        client.init(token);

        let user = client.fetch_user(1087783849183940708, 607197619193643029).await;
        dbg!(&user);

        assert_eq!(user.unwrap().id, 607197619193643029);
        let users = client.fetch_users(1087783849183940708, vec![790507101868654602, 607197619193643029]).await;

        dbg!(&users);
        assert_eq!(users.unwrap().get_user(607197619193643029).unwrap().id, 607197619193643029);

        let lb = client.fetch_leaderboard(1087783849183940708, None, None, None, Some(5)).await;
        dbg!(&lb);

        assert_eq!(lb.unwrap().count, 5);
        let rewards = client.fetch_rewards(1087783849183940708, None, Some(5)).await;

        dbg!(&rewards);
        assert_eq!(rewards.unwrap().count, 5);
    }
}