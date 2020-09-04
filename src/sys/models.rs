use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;

/// system user for login
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Option<i32>,
    pub avatar: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub name: Option<String>,
    pub birthday: Option<String>,
    pub sex: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub create_time: Option<String>,
    pub create_user: Option<i32>,
    pub update_time: Option<String>,
    pub update_user: Option<i32>,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
