#[macro_use] // 何かわからん
extern crate diesel;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, Error, http::header,};
mod db;
mod models;
mod schema;
use crate::models::Dishes;
use crate::models::CountDishes;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!")
}

#[get("/dishes/{id}")]
async fn get(db: web::Data<db::Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // db connectionが利用可能に！
    let conn = db.get().unwrap();
    let id = id.into_inner();

    let dish = schema::dishes::table
        .select((schema::dishes::id, schema::dishes::name, schema::dishes::image, schema::dishes::content))
        .filter(schema::dishes::id.eq(id))
        .load::<Dishes>(&conn)
        .expect("error");

    Ok(HttpResponse::Ok().json(&dish[0]))
}

#[get("/countdishes")]
async fn count(db: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    // db connectionが利用可能に！
    let conn = db.get().unwrap();
    let count: i64 = 
        schema::dishes::table
        .count()
        .get_result(&conn).unwrap();

    Ok(HttpResponse::Ok().json(&count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // db moduleからestablish_connection関数をimport
    let pool = db::establish_connection();
    HttpServer::new(move|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )    
            .app_data(Data::new(pool.clone()))
            .service(get)
            .service(count)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}