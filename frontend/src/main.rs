mod finder;
mod joke;

use finder::*;
use joke::*;

use std::collections::HashSet;

extern crate serde;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub type JokeResult = Result<JokeStruct, gloo_net::Error>;

struct App {
    joke: JokeResult,
}

pub enum Msg {
    GotJoke(JokeResult),
    GetJoke(Option<String>),
}

impl App {
    fn refresh_joke(ctx: &Context<Self>, key: Option<String>) {
        let got_joke = JokeStruct::get_joke(key);
        ctx.link().send_future(got_joke);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App::refresh_joke(ctx, None);
        let joke = Err(gloo_net::Error::GlooError("Loading Joke…".to_string()));
        Self { joke }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotJoke(joke) => {
                self.joke = joke;
                true
            }
            Msg::GetJoke(key) => {
                App::refresh_joke(ctx, key);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let joke = &self.joke;
        html! {
        <>
            <h1>{ "Knock-Knock" }</h1>
            if let Ok(ref joke) = joke {
                <Joke joke={joke.clone()}/>
            }
            if let Err(ref error) = joke {
                <div>
                    <span class="error">{format!("Server Error: {error}")}</span>
                </div>
            }
            <div>
                <button onclick={ctx.link().callback(|_| Msg::GetJoke(None))}>{"Tell me another!"}</button>
            </div>
            <Finder on_find={ctx.link().callback(Msg::GetJoke)}/>
        </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
