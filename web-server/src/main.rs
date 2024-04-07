use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use gcd::Gcd;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(post_gcd)
    });

    println!("Serving on http://localhost:3000");

    server.bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[get("/")]
async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.")
    }

    let response = format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n, form.m, form.n.gcd(form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}