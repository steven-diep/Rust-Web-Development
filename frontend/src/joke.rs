use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct JokeStruct {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl JokeStruct {
    pub async fn get_joke(key: Option<String>) -> Msg {
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
            Err(e) => Msg::GotJoke(Err(e)),
            Ok(data) => Msg::GotJoke(data.json().await),
        }
    }
}
pub fn format_tags(tags: &Vec<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct JokeProps {
    pub joke: JokeStruct,
}

#[function_component(Joke)]
pub fn joke(joke: &JokeProps) -> Html {
    let joke = &joke.joke;
    html! { <>
        <div class="joke">
            <span class="teller">{joke.title.clone()}</span><br/>
            <span class="teller">{joke.content.clone()}</span><br/>
        </div>
        <span class="annotation">
            {format!("[id: {}", &joke.id)}
            if let Some(ref tags) = joke.tags {
                {format!("; tags: {}", &format_tags(tags))}
            }
            {"]"}
        </span>
    </> }
}
