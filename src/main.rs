#[macro_use] extern crate rocket;

use rocket::{serde::{json::Json, Deserialize, Serialize} };   
use uuid::Uuid;
#[cfg(test)] mod test;


#[derive(FromForm, Deserialize, Serialize)]
struct Name {
    game_host: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct Answers {
    text: String,
    is_correct: bool,
}
#[derive(Serialize, Deserialize, Clone)]
struct Question {
    text: String,
    answers: Vec<Answers>,
}
#[derive(Serialize, Deserialize, Clone)]
struct Quiz {
    title: String,
    questions: Vec<Question>,
}
#[derive(Serialize, Deserialize)]
struct Session {
    id: Uuid,
    game_host: String,
    quiz: Quiz
}

pub static BASEURL:&str = "/v1";

#[post("/session", format="application/json", data="<game_host>")]
fn start_session(game_host: Json<Name>) -> Json<Session> {
    let first_answer = Answers { text: "No".to_string(), is_correct: true };
    let second_answer = Answers { text: "Yes".to_string(), is_correct: false };
    let a_question = Question { text: "Is rust the easiest language?".to_string(), answers: vec![first_answer, second_answer] };
    let base_quiz: Quiz = Quiz {
        title: "Best quiz!".to_string(),
        questions: vec![
            a_question
            ],
    };
    let new_session = Session { id: (Uuid::new_v4()), game_host: (game_host.game_host.clone()), quiz: base_quiz.clone() };
    return Json(new_session)
}

#[launch]
pub fn start() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8080))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment).mount(BASEURL, routes![start_session])
}
