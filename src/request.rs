use crate::{user, CCashError, CCashResponse, CCashSession};
use reqwest::{Client, Method};
use serde::Serialize;

fn get_client(session: &CCashSession) -> Result<Client, CCashError> {
    if !session.is_connected() {
        return Err(CCashError::ConnectionNotAvailable);
    }
    Ok(session.get_client().clone().unwrap())
}

pub async fn request<Body: Serialize>(
    method: Method,
    session: &CCashSession,
    uri: &str,
    user: Option<&user::CCashUser>,
    body: Option<&Body>,
) -> Result<CCashResponse, CCashError> {
    let client = get_client(session)?;

    let mut builder = client
        .request(method, uri)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");

    if let Some(user) = user {
        builder = builder.basic_auth(&user.username, Some(&user.password));
    }
    if let Some(body) = body {
        builder = builder.json(&body);
    }

    match client.execute(builder.build()?).await {
        Ok(r) => Ok(CCashResponse::from_response(r).await),
        Err(e) => Err(e.into()),
    }
}
