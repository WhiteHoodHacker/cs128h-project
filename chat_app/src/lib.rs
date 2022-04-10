use rocket::FromForm;
use rocket::serde::*;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Message {
    pub user_id: i32,
    pub user_name: String,
    pub text: String,
    pub timestamp: String,
}
impl Message {
    pub fn as_string(&self) -> String {
        format!(
            "User Id: {}\nUser Name: {}\nText: {}\nDate-Time: {}",
            self.user_id,
            self.user_name,
            self.text,
            self.timestamp
        )
    }
}
