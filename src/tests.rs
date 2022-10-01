use serde_json::json;

use super::*;

pub struct MockAPI;

#[derive(Debug)]
pub struct MockError;

#[async_trait]
impl API for MockAPI {
    type Error = MockError;

    async fn method<T>(&self, method: Method) -> Result<T, Self::Error>
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
