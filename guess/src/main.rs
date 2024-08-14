use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use rand::Rng;
use std::cmp::Ordering;


#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <meta charset="UTF-8">
                <title>猜猜數字</title>
                <h2>數字猜猜猜</h2>
                <p>輸入1~10的數字</p>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <button type="submit">猜!!!</button>
                </form>
            "#,
        )
}


#[derive(Deserialize)]
struct guessor {
    n: u32,
}

async fn post_gcd(form: web::Form<guessor>) -> HttpResponse {

    let secret_number = rand::thread_rng().gen_range(1..=10);

    let response =
        format!("{}",
                guess(form.n, secret_number));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn guess(n: u32, secret_number: u32) -> String {
    match n.cmp(&secret_number) {
        Ordering::Less => "Too small!".to_string(),
        Ordering::Greater => "Too Big!".to_string(),
        Ordering::Equal => "Congraduation!".to_string(),
  }
}

