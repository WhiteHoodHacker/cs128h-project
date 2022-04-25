#[macro_use]
extern crate rocket;
use chat_app::{Message, UserInput, init_db, insert_message, get_messages};
use rocket::Build;
use rocket::fs::{FileServer,relative};
use rocket::response::content::Html;
use rocket::tokio::sync::broadcast::{channel};
use rocket::form::Form;
#[post("/", data = "<form>")]
fn post(form: Form<UserInput>) -> Html<String> {
    let db_file = "db.sqlite";
    let message: Message = form.into_inner().to_message();
    let _temp_ = insert_message(&db_file, &message);
    Html(format!(
        "
        <!DOCTYPE html>
        <html>
            <head>
                <title>Sent</title>
            </head>
            <body>
                <h1>Sent</h1>
                <p>Your message was processed successfully! 🎉</p>
                <p>Contents: {}</p>
                <br>
                <a href=\"/\">Send Another Message</a>
                <br>
                <a href=\"/view\">View Message Logs</a>
            </body>
        </html>
        ",
        message.text
    ))
}
#[get("/")]
fn get() -> Html<String> {
    let db_file = "db.sqlite";
    let messages = get_messages(&db_file).unwrap();
    let mut result = String::new();
    for message in messages {
        result.push_str(&message.as_html());
        result.push_str("<br>");
    }
    Html(format!(
        "
        <!DOCTYPE html>
        <html>
            <head>
                <title>Logs</title>
            </head>
            <body>
                <h1>Message History</h1>
                <br>
                <a href=\"/\">Send Another Message</a>
                <h2>Logs</h2>
                <p>{}</p>
            </body>
        </html>
        ",
        result
    ))
}
#[launch]
fn rocket() -> rocket::Rocket<Build> {
    let db_file = "db.sqlite";
    let _init_result = init_db(&db_file).unwrap();
    rocket::build()
    .manage(channel::<UserInput>(10).0)
    .mount("/send", routes![post])
    .mount("/view", routes![get])
    .mount("/", FileServer::from(relative!("static")))
}
