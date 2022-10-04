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
        UsersGetBuilder::new(&self)
    }

    fn friends_get(&self) -> FriendsGetBuilder<Self> where Self: Sized {
        FriendsGetBuilder::new(&self)
    }
}

pub struct UsersGetBuilder<'a, A: API> {
    pub user_ids: Option<Vec<UserId>>,
    pub user_id: Option<UserId>,
    api: &'a A
}

impl<'a, A: API> UsersGetBuilder<'a, A> {
    fn new(api: &'a A) -> Self {
        UsersGetBuilder {
            user_ids: None,
            user_id: None,
            api
        }
    }

    pub fn user_id(mut self, id: UserId) -> Self {
        self.user_id = Some(id);
        self
    }

    pub fn user_ids(mut self, mut ids: Vec<UserId>) -> Self {
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

    pub async fn send(self) -> Result<Vec<User>, A::Error> {
        let mut params = Params::new();

        if let Some(value) = self.user_id {
            params.insert("user_id", value);
        }

        if let Some(value) = self.user_ids {
            params.insert("user_id", value);
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
    id: UserId,
    first_name: String
}

#[cfg(test)]
mod tests;