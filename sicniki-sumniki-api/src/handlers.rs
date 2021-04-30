use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::service;

#[derive(Serialize)]
struct GetWordResponse {
    word: String,
    similar_words: Vec<String>,
}

#[rocket::get("/words/<word>")]
fn get_word(word: String) -> Result<Json<GetWordResponse>, Box<dyn std::error::Error>> {
    let response = GetWordResponse {
        word: word.to_string(),
        similar_words: service::get_similar_words(&word)?,
    };
    Ok(Json(response))
}

#[derive(Deserialize)]
struct SaveWordRequest {
    word: String,
}

#[rocket::post("/words", data = "<request>")]
fn save_word(request: Json<SaveWordRequest>) -> Result<Status, Box<dyn std::error::Error>> {
    service::save_word(&request.word)?;
    Ok(Status::Created)
}

#[derive(Deserialize)]
struct FixTextRequest {
    text: String,
}

#[derive(Serialize)]
struct FixTextResponse {
    text: String,
}

#[rocket::post("/text/fix", data = "<request>")]
fn fix_text(
    request: Json<FixTextRequest>,
) -> Result<Json<FixTextResponse>, Box<dyn std::error::Error>> {
    let text = service::fix_text(&request.text)?;
    let response = FixTextResponse { text };
    Ok(Json(response))
}

#[derive(Deserialize)]
struct ConfirmTextRequest {
    text: String,
}

#[rocket::post("/text/confirm", data = "<request>")]
fn confirm_text(request: Json<ConfirmTextRequest>) -> Result<Status, Box<dyn std::error::Error>> {
    service::confirm_text(&request.text)?;
    Ok(Status::Created)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_word, save_word, fix_text, confirm_text,]
}
