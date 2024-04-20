mod question;

use question::*;

#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string()))
    );
    println!("{:?}", question);
}
