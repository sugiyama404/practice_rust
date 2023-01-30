#[macro_use]
extern crate diesel;
#[allow(unused_imports)]
use diesel::ExpressionMethods;
#[allow(unused_imports)]
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::web::Data;
use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
mod db;
mod model;
mod schema;

#[get("/api/todo")]
async fn todo_index(db: web::Data<db::Pool>) -> impl Responder {

    let mut conn = db.get().unwrap();
    let todo = schema::todos::table
        .load::<model::Todo>(&mut conn)
        .expect("Error not showing todo list");

    //Ok::<actix_web::web::Json<Vec<model::Todo>>, E>(web::Json(todo))
    //Ok(web::Json(todo))
    HttpResponse::Ok().json(todo)
}

#[post("/api/todo")]
async fn new_todo(db: web::Data<db::Pool>, c: web::Json<model::Updatetodo>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let new_todo = model::Updatetodo {
        content: c.content.to_string(),
    };
    diesel::insert_into(schema::todos::dsl::todos)
        .values(&new_todo)
        .execute(&mut conn)
        .expect("Error not saving new todo");

    HttpResponse::Created().body("get ok")
}

#[delete("/api/todo/{id}")]
async fn delete_todo(db: web::Data<db::Pool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = db.get().unwrap();
    let target = schema::todos::dsl::todos
                    .filter(schema::todos::dsl::id.eq(id));

    diesel::delete(target)
        .execute(&mut conn)
        .expect("Error deleting new post");

    HttpResponse::Created().body("Delete complete")
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = db::establish_connection();
    HttpServer::new(move|| {
        App::new()
        .app_data(Data::new(pool.clone()))
        .service(index)
        .service(todo_index)
        .service(new_todo)
        .service(delete_todo)
    })
    .bind(("web", 8080))?
    .run()
    .await
}
