use std::collections::HashMap;

use reqwest::{
    ClientBuilder, 
    header::{self, HeaderMap}, 
    Client
};

use crate::defs::{
    Leaderboard, Rewards, User, Users, BASE_URL
};


/// The client used to make requests to the Amari API.
/// 
/// # Examples
/// 
/// ```rust
/// use amari_rs::api::AmariClient;
/// use std::env;
/// 
/// let mut client = AmariClient::default();
/// client.init(env::var("AMARI_TOKEN").unwrap());
/// ```
#[derive(Debug, Clone, Default)]
pub struct AmariClient {
    token: String,
    client: Client,
}

impl AmariClient {
    pub fn init(&mut self, token: String) {
        self.token = token;
        self.client = self.request_client();
    }

    pub async fn fetch_user(&self, guild_id: u64, user_id: u64) -> Result<User, reqwest::Error> {
        let url = format!("{BASE_URL}/guild/{guild_id}/member/{user_id}");
        let data = self.client.get(url).send().await.unwrap();

        data.json::<User>().await
    }

    pub async fn fetch_users(&self, guild_id: u64, user_ids: Vec<u64>) -> Result<Users, reqwest::Error> {
        let mut body = HashMap::new();
        let user_ids: Vec<String> = user_ids.iter().map(|&x| x.to_string()).collect();

        body.insert("members", user_ids);
        let url = format!("{BASE_URL}/guild/{guild_id}/members");

        let data = self.client.post(url)
        .json(&body)
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .await.unwrap();

        data.json::<Users>().await
    }

    pub async fn fetch_leaderboard(
        &self, 
        guild_id: u64, 
        weekly: Option<bool>, 
        raw: Option<bool>,
        page: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Leaderboard, reqwest::Error> {
        let mut params = HashMap::new();
        let weekly = weekly.unwrap_or(false);

        if raw.is_some() && page.is_some() {
            panic!("raw endpoint does not support pagination.");
        }

        if page.is_some() {
            params.insert("page", page.unwrap());
        }

        if limit.is_some() {
            params.insert("limit", limit.unwrap());
        }

        let lb_type = if weekly { "weekly" } else { "leaderboard" };
        let url = format!("{BASE_URL}/guild/{lb_type}/{guild_id}");
        

        let data = self.client.get(url).query(&params).send().await.unwrap();
        data.json::<Leaderboard>().await
    }

    pub async fn fetch_rewards(
        &self, 
        guild_id: u64, 
        page: Option<usize>, 
        limit: Option<usize>
    ) -> Result<Rewards, reqwest::Error> {
        let mut params = HashMap::new();
        let page = page.unwrap_or(1);

        let limit = limit.unwrap_or(50);
        let url = format!("{BASE_URL}/guild/rewards/{guild_id}");

        params.insert("page", page);
        params.insert("limit", limit);

        let data = self.client.get(url).query(&params).send().await.unwrap();
        data.json::<Rewards>().await
    }

    fn request_client(&self) -> Client {
        let client: ClientBuilder = ClientBuilder::new();
        let mut default_header = HeaderMap::new();

        default_header.insert(header::AUTHORIZATION, self.token.parse().unwrap());
        client.default_headers(default_header).build().unwrap()
    }
}