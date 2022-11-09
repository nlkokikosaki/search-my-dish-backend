#[macro_use] // 何かわからん
extern crate diesel;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, http::header, web, App, Error, HttpResponse, HttpServer, Responder};
mod db;
mod models;
mod schema;
use crate::models::Dishes;
use schema::dishes;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/dish/{id}")]
async fn get_dish(db: web::Data<db::Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // db connectionが利用可能に！
    let conn = db.get().unwrap();
    let id = id.into_inner();

    let dish = schema::dishes::table
        .select((
            schema::dishes::id,
            schema::dishes::name,
            schema::dishes::image,
            schema::dishes::content,
        ))
        .filter(schema::dishes::id.eq(id))
        .load::<Dishes>(&conn)
        .expect("error");

    Ok(HttpResponse::Ok().json(&dish[0]))
}

#[get("/dishes/{ids}")]
async fn get_dishes(
    db: web::Data<db::Pool>,
    path_ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    // db connectionが利用可能に！
    let conn = db.get().unwrap();
    let path_ids = path_ids.into_inner();
    let str_ids: Vec<String> = path_ids.split(',').fold(Vec::new(), |mut s, i| {
        s.push(i.to_string());
        s
    });
    let ids: Vec<i32> = str_ids.iter().map(|id| id.parse().unwrap()).collect();
    let dishes = dishes::table
        .select((dishes::id, dishes::name, dishes::image, dishes::content))
        .filter(dishes::id.eq(&ids[0]))
        .or_filter(dishes::id.eq(&ids[1]))
        .or_filter(dishes::id.eq(&ids[2]))
        .load::<Dishes>(&conn)
        .expect("error");

    Ok(HttpResponse::Ok().json(&dishes))
}

#[get("/countdishes")]
async fn count(db: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    // db connectionが利用可能に！
    let conn = db.get().unwrap();
    let count: Vec<i32> = schema::dishes::table
        .select(dishes::id)
        .load::<i32>(&conn)
        .expect("error");

    Ok(HttpResponse::Ok().json(&count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // db moduleからestablish_connection関数をimport
    let pool = db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(Data::new(pool.clone()))
            // .service(get_dish)
            .service(get_dishes)
            .service(count)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
