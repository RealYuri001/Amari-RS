use reqwest::{
    header::{self, HeaderMap},
    Client, ClientBuilder,
};
use std::{collections::HashMap, sync::Arc};

use crate::cache::Cache;
use crate::defs::{FetchType, Leaderboard, Rewards, User, Users, BASE_URL};

/// The client used to make requests to the Amari API.
///
/// # Examples
///
/// ```
/// use amari_rs::api::AmariClient;
/// use std::env;
/// use dotenvy::dotenv;
///
/// dotenv().expect("Failed to load .env file");
///
/// let token = env::var("AMARI_TOKEN").unwrap();
/// let mut client = AmariClient::new(token);
/// ```
#[derive(Debug, Clone)]
pub struct AmariClient {
    client: Client,
    cacher: Cache,
}

impl AmariClient {
    pub fn new<S>(token: S) -> Self
    where
        S: AsRef<str>,
    {
        let client: ClientBuilder = ClientBuilder::new();
        let mut default_header = HeaderMap::new();
        default_header.insert(header::AUTHORIZATION, token.as_ref().parse().unwrap());

        AmariClient {
            client: client.default_headers(default_header).build().unwrap(),
            cacher: Cache::new(60, 256 * 1024 * 1024),
        }
    }

    pub async fn fetch_user(
        &mut self,
        guild_id: u64,
        user_id: u64,
        cache: bool,
    ) -> reqwest::Result<User> {
        let url = format!("{BASE_URL}/guild/{guild_id}/member/{user_id}");
        if cache {
            let key = FetchType::User(guild_id, user_id);
            let data = self.cacher.get(&key);

            if data.is_some() {
                return Ok(data.unwrap().downcast_ref::<User>().unwrap().clone());
            }

            let data = self.client.get(url.clone()).send().await?;
            let conv = data.json::<User>().await?;

            self.cacher.set(&key, Arc::new(conv.clone()));
            return Ok(conv);
        }

        let data = self.client.get(url).send().await?;
        data.json::<User>().await
    }

    pub async fn fetch_users(
        &mut self,
        guild_id: u64,
        user_ids: Vec<u64>,
        cache: bool,
    ) -> reqwest::Result<Users> {
        if cache {
            let mut users: Vec<User> = Vec::new();
            let mut uncached_users: Vec<u64> = Vec::new();

            for user_id in user_ids.clone() {
                let key = FetchType::User(guild_id, user_id);
                if let Some(user) = self.cacher.get(&key) {
                    users.push(user.downcast_ref::<User>().unwrap().clone());
                } else {
                    uncached_users.push(user_id);
                }
            }

            if uncached_users.len() > 0 {
                let converted: Vec<String> =
                    uncached_users.iter().map(|&x| x.to_string()).collect();
                let mut body = HashMap::new();

                body.insert("members", converted);
                let url = format!("{BASE_URL}/guild/{guild_id}/members");

                let send_data = self.client.post(url).json(&body).send().await?;
                let raw_json: Users = send_data.json().await?;

                for user in raw_json.members {
                    let key = FetchType::User(guild_id, user.id);
                    self.cacher.set(&key, Arc::new(user.clone()));

                    users.push(user);
                }
            }

            let conv = Users {
                members: users.clone(),
                total_members: users.len(),
                queried_members: user_ids.len(),
                guild_id,
            };

            return Ok(conv);
        }

        let mut body = HashMap::new();
        let user_ids: Vec<String> = user_ids.iter().map(|&x| x.to_string()).collect();

        body.insert("members", user_ids);
        let url = format!("{BASE_URL}/guild/{guild_id}/members");

        let data = self.client.post(url).json(&body).send().await?;
        data.json::<Users>().await
    }

    pub async fn fetch_leaderboard(
        &mut self,
        guild_id: u64,
        weekly: Option<bool>,
        raw: Option<bool>,
        page: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Leaderboard, reqwest::Error> {
        // Doesn't support caching until caching system is improved.

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
        &mut self,
        guild_id: u64,
        page: Option<u32>,
        limit: Option<u32>,
        cache: bool,
    ) -> Result<Rewards, reqwest::Error> {
        if cache {
            let key = FetchType::Reward(guild_id, page.unwrap_or(1), limit.unwrap_or(50));
            if let Some(rewards) = self.cacher.get(&key) {
                return Ok(rewards.downcast_ref::<Rewards>().unwrap().clone());
            }
        }

        let mut params = HashMap::new();
        let page = page.unwrap_or(1);

        let limit = limit.unwrap_or(50);
        let url = format!("{BASE_URL}/guild/rewards/{guild_id}");

        params.insert("page", page);
        params.insert("limit", limit);

        let data = self.client.get(url).query(&params).send().await.unwrap();
        data.json::<Rewards>().await
    }
}

unsafe impl Send for AmariClient {}
unsafe impl Sync for AmariClient {}
