use rocket::FromForm;
use rocket::serde::*;
use chrono;
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
    pub fn as_string(&self) -> String {
        format!(
            "User Id: {}\nUser Name: {}\nText: {}\nDate-Time: {}",
            self.user_id,
            self.user_name,
            self.text,
            self.timestamp.with_timezone(&chrono::Local)
        )
    }
}
