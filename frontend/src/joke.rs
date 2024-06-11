use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct JokeStruct {
    pub id: String,
    pub whos_there: String,
    pub answer_who: String,
    pub tags: Option<HashSet<String>>,
    pub source: Option<String>,
}

impl JokeStruct {
    pub async fn get_joke(key: Option<String>) -> Msg {
        let host = include_str!("../api-url.txt").trim();
        let request = match &key {
            None => format!(
                "{}/api/v1/joke",
                host,
            ),
            Some(ref key) => format!(
                "{}/api/v1/joke/{}",
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
pub fn format_tags(tags: &HashSet<String>) -> String {
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
            <span class="teller">{"Knock-Knock!"}</span><br/>
            <span class="tellee">{"Who's there?"}</span><br/>
            <span class="teller">{joke.whos_there.clone()}</span><br/>
            <span class="tellee">{format!("{} who?", &joke.whos_there)}</span><br/>
            <span class="teller">{joke.answer_who.clone()}</span>
        </div>
        <span class="annotation">
            {format!("[id: {}", &joke.id)}
            if let Some(ref tags) = joke.tags {
                {format!("; tags: {}", &format_tags(tags))}
            }
            if let Some(ref source) = joke.source {
                {format!("; source: {}", source)}
            }
            {"]"}
        </span>
    </> }
}
