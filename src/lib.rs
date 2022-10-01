use serde::Deserialize;
use vk_method::{Method, Params};

pub trait API {
    type Error;

    fn method<T>(&self, method: Method) -> Result<T, Self::Error>
    where for<'de>
        T: serde::Deserialize<'de>;

    fn users_get(&self) -> UsersGetBuilder<Self> where Self: Sized {
        UsersGetBuilder::new(&self)
    }
}

pub struct UsersGetBuilder<'a, A: API> {
    pub user_ids: Option<Vec<i32>>,
    pub user_id: Option<i32>,
    api: &'a A
}

impl<'a, A: API> UsersGetBuilder<'a, A> {
    fn new(api: &'a A) -> UsersGetBuilder<'a, A> {
        UsersGetBuilder {
            user_ids: None,
            user_id: None,
            api
        }
    }

    pub fn user_id(mut self, id: i32) -> UsersGetBuilder<'a, A> {
        self.user_id = Some(id);
        self
    }

    pub fn user_ids(mut self, mut ids: Vec<i32>) -> UsersGetBuilder<'a, A> {
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

    pub fn send(self) -> Result<Vec<User>, A::Error> {
        let mut params = Params::new();

        if let Some(value) = self.user_id {
            params.insert("user_id", value);
        }

        if let Some(value) = self.user_ids {
            params.insert("user_id", value);
        }

        self.api.method(
            Method::new("users.get", params)
        )
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    id: i32,
    first_name: String
}

#[cfg(test)]
mod tests;