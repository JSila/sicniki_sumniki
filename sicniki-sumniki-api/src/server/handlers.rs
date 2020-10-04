use iron::{*, status::Status, headers::ContentType};
use serde::{Deserialize, Serialize};

use crate::{Repository, Service};
use router::Router;

#[derive(Deserialize, Clone, Debug)]
struct FixTextRequest {
    text: String
}

#[derive(Serialize, Debug)]
struct FixTextResponse {
    text: String
}

#[derive(Deserialize, Clone, Debug)]
struct SaveWordRequest {
    word: String
}

#[derive(Deserialize, Clone, Debug)]
struct ConfirmTextRequest {
    text: String
}

pub fn fix_text(req: &mut Request) -> IronResult<Response> {
    let request_data = match req.get::<bodyparser::Struct<FixTextRequest>>() {
        Ok(Some(b)) => b,
        _ => {
            return Ok(respond_bad_request(""))
        }
    };

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let repo = Repository::new(&db_url);
    let s = Service::new(&repo);

    let fixed_text = match s.fix_text(&request_data.text) {
        Ok(t) => t,
        Err(_) => {
            return Ok(respond_internal_server_error(""))
        }
    };

    let response_data = FixTextResponse{text:fixed_text};
    let response = serde_json::to_string(&response_data).unwrap();

    Ok(respond_ok(&response))
}

pub fn get_word(r: &mut Request) -> IronResult<Response> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let repo = Repository::new(&db_url);
    let s = Service::new(&repo);

    let ref word = match r.extensions.get::<Router>()
        .unwrap()
        .find("word") {
        Some(w) => w,
        None => {
            return Ok(respond_bad_request(""))
        }
    };

    let word_data = match s.get_word_data(word) {
        Ok(w) => w,
        Err(_) => {
            return Ok(respond_internal_server_error(""))
        }
    };

    let response = serde_json::to_string(&word_data).unwrap();

    Ok(respond_ok(&response))
}

pub fn save_word(req: &mut Request) -> IronResult<Response> {
    let request_data = match req.get::<bodyparser::Struct<SaveWordRequest>>() {
        Ok(Some(b)) => b,
        _ => {
            return Ok(respond_bad_request(""))
        }
    };

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let repo = Repository::new(&db_url);
    let s = Service::new(&repo);

    if let Err(_) = s.save_word(&request_data.word) {
        return Ok(respond_internal_server_error(""))
    };

    Ok(respond_created())
}

pub fn confirm_text(req: &mut Request) -> IronResult<Response> {
    let request_data = match req.get::<bodyparser::Struct<ConfirmTextRequest>>() {
        Ok(Some(b)) => b,
        _ => {
            return Ok(respond_bad_request(""))
        }
    };

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let repo = Repository::new(&db_url);
    let s = Service::new(&repo);

    if let Err(_) = s.confirm_text(&request_data.text) {
        return Ok(respond_internal_server_error(""))
    };

    Ok(respond_ok(""))
}

pub fn respond_ok(body: &str) -> Response {
    Response::with((ContentType::json().0, Status::Ok, body))
}

pub fn respond_bad_request(body: &str) -> Response {
    Response::with((Status::BadRequest, body))
}

pub fn respond_internal_server_error(body: &str) -> Response {
    Response::with((Status::InternalServerError, body))
}

pub fn respond_created() -> Response {
    Response::with(Status::Created)
}