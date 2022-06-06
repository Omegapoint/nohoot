#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};   
#[cfg(test)] mod test;


#[derive(FromForm, Deserialize, Serialize)]
struct Name {
    name: String,
}

pub static BASEURL:&str = "/v1";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!\n"
}

#[post("/session", format="application/json", data="<name>")]
fn start_session(name: Json<Name>) -> String {
    let r = format!("You get a session for {}!\n", name.name.as_str());
    return r;
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(BASEURL, routes![index, start_session])
}