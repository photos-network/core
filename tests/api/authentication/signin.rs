//! Test authentication behaviour like invalid user or password
//!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 3;
        let b = 1 + 1;
        
        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }

    #[tokio::test]
    async fn authenticate_without_user() {
        // given
        let test_state = spawn_app().await;
        let client = test_state.api_client;
        let input = serde_json::json!({"password": "secret"});

        // when
        let response = client
            .post(&format!("{}/oauth/authorize", &test_state.app_address))
            .form(&input)
            .send()
            .await
            .expect("authorization request failed!");

        // then
        assert_eq!(
            response.status().as_u16(),
            422,
            "{} returns client error status",
            "no username"
        );
    }
}
