mod vk_date_format;
mod vk_date_format_opt;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
pub use vk_method::{Method, Params};
use async_trait::async_trait;

pub type UserId = u32;

#[async_trait]
pub trait API {
    type Error: std::error::Error;

    async fn method<T>(&self, method: Method) -> Result<T, Self::Error>
    where for<'de>
        T: serde::Deserialize<'de>;

    fn users_get(&self) -> UsersGetBuilder<Self> where Self: Sized {
        UsersGetBuilder::new(self)
    }

    fn friends_get(&self) -> FriendsGetBuilder<Self> where Self: Sized {
        FriendsGetBuilder::new(self)
    }
}

pub struct UsersGetBuilder<'a, A: API> {
    pub user_ids: Option<Vec<String>>,
    pub user_id: Option<String>,
    pub fields: Option<Vec<UsersFields>>,
    api: &'a A
}

impl<'a, A: API> UsersGetBuilder<'a, A> {
    fn new(api: &'a A) -> Self {
        UsersGetBuilder {
            user_ids: None,
            user_id: None,
            fields: None,
            api
        }
    }

    pub fn user_id(mut self, id: impl ToString) -> Self {
        self.user_id = Some(id.to_string());
        self
    }

    pub fn user_ids(mut self, mut ids: Vec<impl ToString>) -> Self {
        let mut ids = ids.iter_mut().map(|id| id.to_string()).collect();

        match self.user_ids {
            Some(ref mut users) => {
                users.append(&mut ids)
            },
            None => {
                self.user_ids = Some(ids);
            }
        }
        self
    }
    
    pub fn fields(mut self, fields: Vec<UsersFields>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub async fn send(self) -> Result<Vec<User>, A::Error> {
        let mut params = Params::new();

        if let Some(value) = self.user_id {
            params.insert("user_id", value);
        }

        if let Some(value) = self.user_ids {
            params.insert("user_id", value);
        }

        if let Some(value) = self.fields {
            params.insert("fields", value.into_iter().map(|field| field.to_string()).collect::<Vec<String>>());
        }
        
        self.api.method(
            Method::new("users.get", params)
        ).await
    }
}

pub struct FriendsGetBuilder<'a, A: API> {
    pub user_id: Option<UserId>,
    pub count: Option<u16>,
    api: &'a A
}

impl<'a, A: API> FriendsGetBuilder<'a, A> {
    fn new(api: &'a A) -> Self {
        FriendsGetBuilder {
            user_id: None,
            count: None,
            api
        }
    }

    pub fn user_id(mut self, id: UserId) -> Self {
        self.user_id = Some(id);
        self
    }

    pub fn count(mut self, count: u16) -> Self {
        self.count = Some(count);
        self
    }

    pub async fn send(self) -> Result<FriendsGetResponse, A::Error> {
        let mut params = Params::new();

        if let Some(value) = self.user_id {
            params.insert("user_id", value);
        }

        if let Some(value) = self.count {
            params.insert("count", value);
        }

        self.api.method(
            Method::new("friends.get", params)
        ).await
    }
}

#[derive(Serialize, Deserialize)]
pub struct FriendsGetResponse {
    pub count: u16,
    pub items: Vec<UserId>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: UserId,
    pub first_name: String,
    #[serde(default)]
    #[serde(with = "vk_date_format_opt")]
    pub bdate: Option<NaiveDate>
}

#[derive(strum::Display)]
#[derive(Serialize)]
pub enum UsersFields {
    bdate
}

#[cfg(test)]
mod tests;
