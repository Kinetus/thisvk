use serde_json::json;

use super::*;

pub struct MockAPI;

#[derive(Debug)]
pub struct MockError;

impl API for MockAPI {
    type Error = MockError;

    fn method<T: serde::de::DeserializeOwned>(
        &self,
        _method: Method,
    ) -> Result<T, Self::Error> {
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

#[test]
fn test() {
    let api = MockAPI;

    assert_eq!(
        api.users_get().user_id(1).user_ids(vec![4,5]).send().unwrap(),
        vec![User {id: 5, first_name: "durov".to_string()}]
    );
}
