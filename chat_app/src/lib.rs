use rocket::FromForm;
use rocket::serde::*;
use chrono;
use rusqlite;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]

pub struct UserInput {
    message: String,
}
impl UserInput {
    pub fn to_message(&self) -> Message {
        Message {
            user_id: 1,
            user_name: String::from("test"),
            text: self.message.clone(),
            timestamp: chrono::Utc::now(),
        }
    }
}
pub struct Message {
    pub user_id: i32,
    pub user_name: String,
    pub text: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
impl Message {
    pub fn new(user_id: i32, user_name: String, text: String) -> Message {
        Message {
            user_id: user_id,
            user_name: user_name,
            text: text,
            timestamp: chrono::Utc::now(),
        }
    }
    pub fn new_from_row(row: &rusqlite::Row) -> Message {
        let rfc_str: String = row.get(4).unwrap();
        let timestamp = chrono::DateTime::parse_from_rfc2822(&rfc_str).unwrap();
        Message {
            user_id: row.get(1).unwrap(),
            user_name: row.get(2).unwrap(),
            text: row.get(3).unwrap(),
            timestamp: chrono::DateTime::<chrono::Utc>::from(timestamp),
        }
    }
    pub fn as_string(&self) -> String {
        format!(
            "User Id: {}\nUser Name: {}\nText: {}\nDate-Time: {}",
            self.user_id,
            self.user_name,
            self.text,
            self.timestamp.with_timezone(&chrono::Local)
        )
    }
    pub fn as_html(&self) -> String {
        format!(
            "<p>User Id: {}</p>\n<p>User Name: {}</p>\n<p>Text: {}</p>\n<p>Date-Time: {}</p>",
            self.user_id,
            self.user_name,
            self.text,
            self.timestamp.with_timezone(&chrono::Local)
        )
    }
}
pub fn init_db(db_file: &str) -> Result<(), rusqlite::Error> {
    let conn = rusqlite::Connection::open(db_file)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            user_name TEXT NOT NULL,
            text TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}
pub fn insert_message(db_file: &str, message: &Message) -> Result<(), rusqlite::Error> {
    let conn = rusqlite::Connection::open(db_file)?;
    let mut tx = conn.prepare(
        "INSERT INTO messages (user_id, user_name, text, timestamp)
        VALUES (:user_id, :user_name, :text, :timestamp)"
    )?;
    let _temp_ = tx.execute(rusqlite::named_params!{
        ":user_id": &message.user_id,
        ":user_name": &message.user_name,
        ":text": &message.text,
        ":timestamp": String::from(&message.timestamp.to_rfc2822())
    });
    Ok(())
}
pub fn get_messages(db_file: &str) -> Result<Vec<Message>, rusqlite::Error> {
    let conn = rusqlite::Connection::open(db_file).unwrap();
    let mut messages = Vec::new();
    let mut tx = conn.prepare("SELECT * FROM messages")?;
    for row in tx.query_map([], |row| {
        Ok(Message::new_from_row(row))
    })? {
        messages.push(row?);
    }
    Ok(messages)
}