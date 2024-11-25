use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Base URL for AmariBot API.
pub const BASE_URL: &'static str = "https://amaribot.com/api/v1";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FetchType {
    /// (guild id, page, limit)
    Leaderboard(u64, u32, u32),
    /// (guild id, page, limit)
    WeeklyLeaderboard(u64, u32, u32),
    /// (guild id, page, limit)
    Reward(u64, u32, u32),
    /// (guild id, user id)
    User(u64, u64),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Leaderboard {
    pub count: u64,
    #[serde(rename = "data")]
    pub users: Vec<User>,
    pub total_count: u64,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u64,
    pub username: String,
    pub exp: u32,
    pub level: Option<u32>,

    #[serde(rename = "weeklyExp")]
    pub weekly_exp: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Users {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "guild")]
    pub guild_id: u64,
    pub members: Vec<User>,
    pub total_members: usize,
    pub queried_members: usize,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RewardRole {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "roleID")]
    pub role_id: u64,
    pub level: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rewards {
    pub count: u64,
    #[serde(rename = "data")]
    pub roles: Vec<RewardRole>,
}

impl Users {
    pub fn get_user(&self, user_id: u64) -> Option<&User> {
        self.members.iter().find(|u| u.id == user_id)
    }

    pub fn len(&self) -> usize {
        self.total_members
    }
}

impl Rewards {
    pub fn get_role(&self, role_id: u64) -> Option<&RewardRole> {
        self.roles.iter().find(|r| r.role_id == role_id)
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }
}
