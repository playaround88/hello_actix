use super::models::User;
use crate::{MyError, DBPOOL};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::prelude::*;

/// route configuration
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      web::resource("/user")
        .route(web::get().to(list))
        .route(web::post().to(save))
        .route(web::put().to(update)), // put:completely update entity, patch:partially update entity
    )
    .service(
      web::scope("/user")
        .route("/{user_id}", web::delete().to(del))
        .route("/{user_id}", web::get().to(get)),
    );
}

/// save a User entity
async fn save(user: web::Json<User>) -> impl Responder {
  HttpResponse::Ok().body(format!("save user:{:?}", user))
}

/// delete a User
async fn del(user_id: web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("deleted user: {}", user_id))
}

/// update a User
async fn update(user: web::Json<User>) -> impl Responder {
  HttpResponse::Ok().body(format!("update user: {:?}", user))
}

/// get a User detail
async fn get(user_id: web::Path<u32>) -> impl Responder {
  HttpResponse::Ok().body(format!("get user: {}", user_id))
}

/// get User list by params
async fn list(req: HttpRequest) -> Result<String, MyError> {
  let current_page: usize = req.match_info().query("_page").parse().unwrap_or(1);
  let page_size: usize = req.match_info().query("_size").parse().unwrap_or(10);
  let mut cursor = sqlx::query("SELECT * from sys_user where user_id=?")
    .bind("1")
    .fetch(&*DBPOOL);
  let mut list = Vec::new();
  while let Some(row) = cursor.next().await? {
    // map the row into a user-defined domain type
    let user = User::from_row(&row)?;
    list.push(user);
  }
  Ok(format!(
    "get user list, current_page:{}, page_size:{}, result: {:?}",
    current_page, page_size, list
  ))
}
