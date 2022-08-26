#[macro_use] // 何かわからん
extern crate diesel;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, Error};
mod db;
mod models;
mod schema;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/dishes/{id}")]
async fn get(db: web::Data<db::Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // db connectionが利用可能に！
    let conn = db.get().unwrap();
    let id = id.into_inner();

    let dish = schema::dishes::table
        .select(schema::dishes::name)
        .filter(schema::dishes::id.eq(id))
        .load::<String>(&conn)
        .expect("error");

    Ok(HttpResponse::Ok().json(dish))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // db moduleからestablish_connection関数をimport
    let pool = db::establish_connection();
    HttpServer::new(move|| {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get)
            .service(hello)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}