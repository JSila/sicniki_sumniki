use sicniki_sumniki::{Repository, Service};

fn main() {
    dotenv::dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let r = Repository::new(&db_url);
    let s = Service::new(&r);

    let text = "Rdečebel čebelar.";
    s.confirm_text(text).unwrap();
}