use log::{error, warn};

use crate::api::{
    constant::USER_AGENT,
    error::{HttpError, ReqwestOrHttpError},
    model::zkill::ZkillStats,
};

const ZKILL_URL: &str = "https://zkillboard.com";
const MAX_ATTEMPTS: u32 = 3;

pub async fn get_zkill_corporation_stats(
    corporation_id: i32,
    api_url: Option<&str>,
) -> Result<ZkillStats, ReqwestOrHttpError> {
    let client: reqwest::Client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let base_url = api_url.unwrap_or(ZKILL_URL);
    let request_url = format!("{}/api/stats/corporationID/{}/", base_url, corporation_id);

    let mut attempt = 0;

    loop {
        attempt += 1;

        let response = match client.get(&request_url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Request failed: {}", e);
                return Err(ReqwestOrHttpError::Reqwest(e));
            }
        };

        match response.status() {
            reqwest::StatusCode::OK => {
                let json: ZkillStats = match response.json().await {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Failed to parse JSON: {}", e);
                        return Err(ReqwestOrHttpError::Reqwest(e));
                    }
                };

                return Ok(json);
            }
            reqwest::StatusCode::GATEWAY_TIMEOUT if attempt < MAX_ATTEMPTS => {
                continue;
            }
            _ => {
                let code = response.status();
                let message = response.text().await.unwrap_or_default();

                warn!(
                    "Failed to get zkill stats for corporation_id {} with status code {}: {}",
                    corporation_id, code, message
                );

                return Err(ReqwestOrHttpError::Http(HttpError {
                    code: code.as_str().to_string(),
                    message,
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_get_zkill_corporation_stats_success() {
        let mut server = Server::new_async().await;

        let base_url = server.url();

        let corporation_id = 12345;

        let mock = server
            .mock(
                "GET",
                format!("/api/stats/corporationID/{}/", corporation_id).as_str(),
            )
            .with_status(200)
            .with_body(r#"{"info": {"member_count": 100}, "ships_destroyed": 50}"#)
            .create();

        let result = get_zkill_corporation_stats(corporation_id, Some(&base_url)).await;

        mock.assert();

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.ships_destroyed, 50);
        assert_eq!(stats.info.member_count, 100);
    }

    #[tokio::test]
    async fn test_get_zkill_corporation_stats_timeout() {
        let mut server = Server::new_async().await;

        let base_url = server.url();

        let corporation_id = 12345;

        let mock = server
            .mock(
                "GET",
                format!("/api/stats/corporationID/{}/", corporation_id).as_str(),
            )
            .with_status(504)
            .expect_at_most(3)
            .create();

        let result = get_zkill_corporation_stats(corporation_id, Some(&base_url)).await;

        mock.assert();

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_zkill_corporation_stats_Failure() {
        let mut server = Server::new_async().await;

        let base_url = server.url();

        let corporation_id = 12345;

        let mock = server
            .mock(
                "GET",
                format!("/api/stats/corporationID/{}/", corporation_id).as_str(),
            )
            .with_status(500)
            .expect_at_most(1)
            .create();

        let result = get_zkill_corporation_stats(corporation_id, Some(&base_url)).await;

        mock.assert();

        assert!(result.is_err());
    }
}
