use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;


struct TodoEntry {
    id: u32,
    text: String
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error)
}

// It will be error on Intellij editor. but works fine with real compiler.
// ref: 実践Rustプログラミング入門 p.193
impl ResponseError for MyError {}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("localhost:8080")?
        .run()
        .await?;
    Ok(())
}

// change actix_web::Error to MyError
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string()
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string()
    });

    let html = IndexTemplate { entries };
    // let response_body = "Hello world!";
    let response_body = html.render()?;

    // HttpResponse::Ok() is a struct of HttpResponseBuilder and it contain status code 200.
    // if input the response body to function body() of HttpResponseBuilder, It will return HttpResponse.
    // The return type is Result, so we wrapped with Ok().
    // Ok(HttpResponse::Ok().body(response_body))
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

