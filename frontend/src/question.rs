use crate::*;

/// Question struct used to represent a question from the backend
#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionStruct {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl QuestionStruct {
    /// Get question from the backend
    pub async fn get_question(key: Option<String>) -> Msg {
        let host = include_str!("../api-url.txt").trim();

        // If a key is passed, get the corresponding question; otherwise, get a random question
        let request = match &key {
            None => format!("{}/question", host,),
            Some(ref key) => format!("{}/question/{}", host, key,),
        };
        // Match the response and process the question if ok
        let response = http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => Msg::GotQuestion(data.json().await),
        }
    }
}

/// Function to format the tags vector as a string
pub fn format_tags(tags: &[String]) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

/// Prop used to store a question
#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionStruct,
}

/// Component to render a question and its content
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
