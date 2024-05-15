use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use validator::Validate;

mod models;
use crate::models::paper::PublishPaperRequest;

#[get("/authors")]
async fn get_authors() -> impl Responder {
    HttpResponse::Ok().body("Authors")
}

#[get("/papers")]
async fn get_papers() -> impl Responder {
    HttpResponse::Ok().body("Papers")
}

#[post("/publish")]
async fn post_publish(body: Json<PublishPaperRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let paper_title = body.paper_title.clone();
            HttpResponse::Ok().body(format!("paper title is {paper_title}"))
        }
        Err(_) => HttpResponse::Ok().body("Invalid request"),
    }
}

#[patch("/update_paper/{uuid}")]
async fn update_paper() -> impl Responder {
    HttpResponse::Ok().body("update paper")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_authors)
            .service(get_papers)
            .service(post_publish)
            .service(update_paper)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
