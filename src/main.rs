mod common;
mod service;
mod entity;
mod api;

use std::env;
use std::fmt::Formatter;
use std::time::Duration;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::ServiceRequest;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Query;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use derive_more::Display;
use env_logger::Builder;
use log::{error, info, warn};
use serde::Deserialize;
use thiserror::Error;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm::DbErr;
use serde_json::Error;
use crate::common::result::CommonResult;
use crate::common::security::{Claims, Security};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "Debug");
    env_logger::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let _ = env::var("SECRET_KEY").expect("SECRET_KEY is not set in .env file");
    let server_url = format!("{host}:{port}");

    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        // .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    let db = Database::connect(opt).await.unwrap();
    let state = AppState {conn: db };

    let server = HttpServer::new(move|| {
        let auth = HttpAuthentication::with_fn(validator);
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(hello)
            .service(echo)
            .service(query)
            .service(test)
            .wrap(auth)
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 3).error_handler(handle_json_error))
            .configure(init_service)
    }).workers(3).bind(&server_url);

    match server {
        Ok(_) => println!("Create Server Successful!"),
        Err(e) => panic!("Create Server Error!{}",e),
    }
    server.unwrap().run().await?;
    Ok(())
}

fn init_service(cfg: &mut web::ServiceConfig) {
    api::dispatch(cfg);
}

// 自定义错误处理程序函数
fn handle_json_error(err: actix_web::error::JsonPayloadError, _req: &HttpRequest)->actix_web::Error {
    // 在这里处理 JSON payload 错误，例如返回适当的错误响应或记录错误日志
    let msg = CommonResult::<String>::fail(400, format!("JSON deserialization error: {}", err)).to_string();
    actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().body(msg).into()).into()
}


#[derive(Error,Debug)]
enum UserError {
    #[error("Validation error on field: {}", field)]
    ValidationError { field: String },
    #[error("Database error occurred")]
    DbErr(#[from] DbErr),
    #[error("json paser error")]
    JsonErr(#[from] serde_json::Error),
    #[error("{0}")]
    Error(String),
}


impl actix_web::error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::JsonErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => {
                println!("User Error: {}", "????");
                StatusCode::INTERNAL_SERVER_ERROR
            },
        }
    }
    fn error_response(&self) -> HttpResponse {
        let msg = match self {
            UserError::ValidationError { field: _ } => self.to_string(),
            UserError::DbErr(e) => e.to_string(),
            UserError::JsonErr(e) => e.to_string(),
            UserError::Error(e) => e.to_string(),
            _ => "Unknown Error".to_string(),
        };
        let res = CommonResult::<String>::fail(400, msg).to_string();
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(res)
    }
}

fn excluded_routes()->Vec<&'static str> {
    vec![
        "/auth/signin"
    ]
}

async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let path = req.path().to_string();
    let routes = excluded_routes();
    if routes.contains(&path.as_str()) {
        return Ok(req);
    }
    let Some(credentials) = credentials else {
        return Err((actix_web::error::ErrorBadRequest("no bearer header"), req));
    };
    let token = credentials.token();
    info!("{:?}",token);
    match Security::decode_token(token) {
        Ok(_) => Ok(req),
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Unauthorized"), req))
    }
}

#[get("/query")]
async fn query(query:Query<DemoPage>) -> Result<String,UserError> {
    info!("{}",query.size);
    info!("{}",query.page);
    if query.page == 10 {
        error!("page 10");
        return Err(UserError::ValidationError{field:"page 10".to_string()});
    }
    Ok(String::from("Hello world!"))
}

#[get("/res")]
async fn test() -> Result<impl Responder,UserError> {
    let string = "123".to_string();
    Ok(CommonResult::success(string))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/")]
async fn hello() -> impl Responder {
    warn!("处理根请求");
    HttpResponse::Ok().body("Hello world!")
}

async fn root_handler() -> &'static str {
    warn!("处理根请求");
    "Hello, Axum!"
}

#[derive(Deserialize)]
struct DemoPage {
    page:usize,
    size:usize
}
