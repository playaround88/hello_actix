extern crate dotenv;
use dotenv::dotenv;
use std::env;
#[macro_use]
extern crate lazy_static;
use futures::executor::block_on;
use sqlx::SqlitePool;
#[macro_use]
extern crate failure;

pub mod comm;
pub mod error;
pub mod sys;

pub use crate::error::MyError;

lazy_static! {
    static ref DBPOOL: SqlitePool = {
        dotenv().ok();
        block_on(
            SqlitePool::builder()
                .max_size(5)
                .build(&env::var("DATABASE_URL").unwrap()),
        )
        .unwrap()
    };
}
