use chat_app::Message;
fn main() {
    let x = Message{
        text: String::from("Hello"),
        user_id: 1,
        user_name: String::from("John"),
        date: String::from("20222-01-01"),
        time: String::from("12:00"),
    };
    println!("{:?}", x);
}
