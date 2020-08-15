use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::LcError;

pub mod question;

pub type ServiceResult<T> = Result<T, LcError>;

pub const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

pub async fn fetch<Q, T>(url: &str, query: &Q) -> ServiceResult<T>
where
    Q: Serialize + ?Sized,
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&query)
        .send()
        .await
        .map_err(|_| LcError::Fetch)?;
    let json = res.json().await.map_err(|_| LcError::Parse)?;
    Ok(json)
}

pub async fn gql_fetch<Q, T>(query: &Q) -> ServiceResult<T>
where
    Q: Serialize + ?Sized,
    T: DeserializeOwned,
{
    fetch::<Q, T>(GRAPHQL_URL, query).await
}
