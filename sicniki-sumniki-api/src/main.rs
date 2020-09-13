use iron::{Chain, Iron};
use iron_cors::CorsMiddleware;
use router::Router;

use sicniki_sumniki::handlers::*;

fn main() {
    dotenv::dotenv().unwrap();

    let mut router = Router::new();

    router.post("/text/fix", fix_text, "fix-text");
    router.post("/text/confirm", confirm_text, "confirm-text");

    router.post("/words", save_word, "save-word");
    router.get("/words/:word", get_word, "get-word");

    let mut chain = Chain::new(router);
    chain.link_around(CorsMiddleware::with_allow_any());

    Iron::new(chain)
        .http(("localhost", 3000))
        .unwrap();
}