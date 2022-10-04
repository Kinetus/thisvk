use std::fmt::Display;

use serde_json::json;

use super::*;

pub struct MockAPI;

#[derive(Debug, thiserror::Error)]
pub struct MockError;

impl Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mock error")
    }
}

#[async_trait]
impl API for MockAPI {
    type Error = MockError;

    async fn method<T>(&self, _method: Method) -> Result<T, Self::Error>
    where for<'de>
        T: serde::Deserialize<'de>
    {
        Ok(
            serde_json::from_value(json!(
                [{
                    "id": 5,
                    "first_name": "durov"
                }]
            )
        ).unwrap())
    }
}

#[tokio::test]
async fn test() {
    let api = MockAPI;

    assert_eq!(
        api.users_get().user_id(1).user_ids(vec![4,5]).send().await.unwrap(),
        vec![User {id: 5, first_name: "durov".to_string()}]
    );
}
