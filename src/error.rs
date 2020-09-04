use failure::Fail;
use std::io;
use std::num;

#[derive(Debug, Fail)]
pub enum MyError {
  #[fail(display = "{}", _0)]
  Io(io::Error),
  #[fail(display = "{}", _0)]
  Parse(num::ParseIntError),
  #[fail(display = "{}", _0)]
  Serde(#[cause] serde_json::Error),
  #[fail(display = "{}", _0)]
  Sql(#[cause] sqlx::Error),
}

impl From<io::Error> for MyError {
  fn from(err: io::Error) -> MyError {
    MyError::Io(err)
  }
}

impl From<serde_json::Error> for MyError {
  fn from(err: serde_json::Error) -> MyError {
    MyError::Serde(err)
  }
}

impl From<num::ParseIntError> for MyError {
  fn from(err: num::ParseIntError) -> MyError {
    MyError::Parse(err)
  }
}

impl From<sqlx::Error> for MyError {
  fn from(err: sqlx::Error) -> MyError {
    MyError::Sql(err)
  }
}

impl actix_web::ResponseError for MyError {}
