use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use sicniki_sumniki::handlers;

fn main() {
    dotenv::dotenv().ok();

    let frontend_app_url = std::env::var("FRONTEND_APP_URL").unwrap();

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[frontend_app_url]),
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .mount("/", handlers::routes())
        .attach(cors)
        .launch();
}
