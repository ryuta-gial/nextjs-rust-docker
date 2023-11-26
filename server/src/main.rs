mod schema;
mod models;

use diesel::{prelude::*, r2d2::{self, ConnectionManager}};
use std::env;
use dotenv::dotenv;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder, http::header};
use serde::Deserialize;

use crate::models::{NewPost};
use crate::schema::posts::dsl::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// リクエストデータの構造体
#[derive(Deserialize)]
struct Person {
    name: String,
    age: i32,
}

// グリーティング関数
async fn greet() -> impl Responder {
    "Hello, world!"
}

// リクエストハンドラ
async fn submit(person: web::Json<Person>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let new_post = NewPost { 
        name: &person.name, 
        age: person.age 
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut conn)
        .expect("Error saving new post");

    let response = format!("Received name: {}, age: {}", person.name, person.age);
    println!("{}", response);
    HttpResponse::Ok().body(response)
}

// メイン関数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .route("/", web::get().to(greet))
            .route("/submit", web::post().to(submit))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
