use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionStruct {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl QuestionStruct {
    pub async fn get_question(key: Option<String>) -> Msg {
        let host = include_str!("../api-url.txt").trim();
        let request = match &key {
            None => format!(
                "{}/question",
                host,
            ),
            Some(ref key) => format!(
                "{}/question/{}",
                host,
                key,
            ),
        };
        let response = http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => Msg::GotQuestion(data.json().await),
        }
    }
}
pub fn format_tags(tags: &[String]) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionStruct,
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class="question">
            <span class="teller">{question.title.clone()}</span><br/>
            <span class="teller">{question.content.clone()}</span><br/>
        </div>
        <span class="annotation">
            {format!("[id: {}", &question.id)}
            if let Some(ref tags) = question.tags {
                {format!("; tags: {}", &format_tags(tags))}
            }
            {"]"}
        </span>
    </> }
}
