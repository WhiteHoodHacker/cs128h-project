#[macro_use]
extern crate rocket;
use chat_app::Message;
use chat_app::UserInput;
use rocket::fs::{FileServer,relative};
use rocket::tokio::sync::broadcast::{channel};
use rocket::form::Form;

#[post("/", data = "<form>")]
fn post(form: Form<UserInput>)-> String {
    let message: Message = form.into_inner().to_message();
    message.as_string()
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(channel::<UserInput>(10).0)
    .mount("/send", routes![post])
    .mount("/", FileServer::from(relative!("static")))
}
