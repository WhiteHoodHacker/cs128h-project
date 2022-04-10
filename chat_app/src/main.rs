#[macro_use]
extern crate rocket;
use chat_app::Message;
use chrono;
use rocket::fs::{FileServer,relative};
use rocket::response::stream::EventStream;
use rocket::{State, Shutdown};
use rocket::tokio::sync::broadcast::{channel, Sender};
use rocket::form::Form;
use rocket::response::stream::Event;
use rocket::tokio::select;
#[get("/")]
fn test() -> String {
    let message = Message {
        user_id: 1,
        user_name: String::from("John"),
        text: String::from("Hello, world!"),
        timestamp: chrono::Utc::now().to_rfc2822(),
    };
    message.as_string()
}
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>){
    let _temp = queue.send(form.into_inner());
}
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(_) => break,
                },
                _ = &mut end => break,
        };
            yield Event::json(&msg);
        }
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(channel::<Message>(10).0)
    .mount("/", routes![post, events])
    .mount("/", FileServer::from(relative!("static")))
}
