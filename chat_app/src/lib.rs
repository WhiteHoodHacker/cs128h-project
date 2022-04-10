use chrono::DateTime;
use chrono::Local;
use chrono::Utc;

#[derive(Debug)]
pub struct Message {
    pub user_id: i32,
    pub user_name: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}
impl Message {
    pub fn as_string(&self) -> String {
        format!(
            "User Id: {}\nUser Name: {}\nText: {}\nDate-Time: {}",
            self.user_id,
            self.user_name,
            self.text,
            self.timestamp.with_timezone(&Local)
        )
    }
}
