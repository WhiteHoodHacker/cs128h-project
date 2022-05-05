#[macro_use]
extern crate rocket;
use chat_app::{get_messages, init_db, insert_message, Message, UserInput};
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::content::Html;
use rocket::tokio::sync::broadcast::channel;
use rocket::Build;
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
        result.push_str("<br line-height:50%>");
    }
    Html(format!(
        "
        <!DOCTYPE html>
        <html>
            <head>
                <!-- Required meta tags -->
                <meta charset=\"utf-8\">
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1, shrink-to-fit=no\">
                <!-- Bootstrap CSS -->
                <link rel=\"stylesheet\" href=\"https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css\" integrity=\"sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T\" crossorigin=\"anonymous\">
                <title>Title</title>
                <link rel=\"stylesheet\" href=\"/reset.css\">
                <link rel=\"stylesheet\" href=\"/style.css\">
                <script src=\"script.js\" charset=\"utf-8\" defer></script>
            </head>
            <body style=\"margin:35px\">
                <div class=\"p-3 mb-2 bg-info text-white\" style=\"border-radius:5px;text-align:center\">
                    <h1>Message History</h1>
                    
                </div>
                <div  style=\"text-align:center\">
                    <a href=\"/\">Send Another Message</a>
                </div>
                <br>
                <div style=\"margin:20px\">
                <h2>Logs</h2>
                <p><small class=\"text-muted\">{}</small></p>
                </div>
                <script src=\"https://code.jquery.com/jquery-3.3.1.slim.min.js\" integrity=\"sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo\" crossorigin=\"anonymous\"></script>
                <script src=\"https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.7/umd/popper.min.js\" integrity=\"sha384-UO2eT0CpHqdSJQ6hJty5KVphtPhzWj9WO1clHTMGa3JDZwrnQq4sF86dIHNDz0W1\" crossorigin=\"anonymous\"></script>
                <script src=\"https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/js/bootstrap.min.js\" integrity=\"sha384-JjSmVgyd0p3pXB1rRibZUAYoIIy6OrQ6VrjIEaFf/nJGzIxFDsf4x0xIM+B07jRM\" crossorigin=\"anonymous\"></script>
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
