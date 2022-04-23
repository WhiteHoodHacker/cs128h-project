#[macro_use]
extern crate rocket;
use chat_app::{Message, UserInput, init_db, insert_message, get_messages};
use rocket::Build;
use rocket::fs::{FileServer,relative};
use rocket::tokio::sync::broadcast::{channel};
use rocket::form::Form;

#[post("/", data = "<form>")]
fn post(form: Form<UserInput>) -> String{
    let db_file = "db.sqlite";
    let message: Message = form.into_inner().to_message();
    let _temp_ = insert_message(&db_file, &message);
    let messages = get_messages(&db_file).unwrap();
    let mut result = String::new();
    for message in messages {
        result.push_str(&message.as_string());
        result.push_str("\n\n");
    }
    result
}
#[launch]
fn rocket() -> rocket::Rocket<Build> {
    let db_file = "db.sqlite";
    let _init_result = init_db(&db_file).unwrap();
    rocket::build()
    .manage(channel::<UserInput>(10).0)
    .mount("/send", routes![post])
    .mount("/", FileServer::from(relative!("static")))
}
