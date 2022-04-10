#[macro_use]
extern crate rocket;
use chat_app::Message;
use chrono;

#[get("/")]
fn test() -> String {
    let message = Message {
        user_id: 1,
        user_name: String::from("John"),
        text: String::from("Hello, world!"),
        timestamp: chrono::Utc::now(),
    };
    message.as_string()
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![test])
}
