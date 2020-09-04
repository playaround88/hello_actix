use actix_web::{Either, Error, HttpResponse};

pub type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;
