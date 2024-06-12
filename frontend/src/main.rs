mod finder;
mod question;

use finder::*;
use question::*;

extern crate serde;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

/// Type to store a question or error
pub type QuestionResult = Result<QuestionStruct, gloo_net::Error>;

/// App struct to render components
struct App {
    question: QuestionResult,
}

/// Enum to either recieve results from a request or send a new request
pub enum Msg {
    GotQuestion(QuestionResult),
    GetQuestion(Option<String>),
}

impl App {
    /// Call the get_question method to send a request and set the context
    fn refresh_question(ctx: &Context<Self>, key: Option<String>) {
        let got_question = QuestionStruct::get_question(key);
        ctx.link().send_future(got_question);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    /// Initialize the component
    fn create(ctx: &Context<Self>) -> Self {
        App::refresh_question(ctx, None);
        let question = Err(gloo_net::Error::GlooError("Loading Question…".to_string()));
        Self { question }
    }

    /// Handle a message and re-render the component if needed
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // Re-render or send a request depending on the message
        match msg {
            Msg::GotQuestion(question) => {
                self.question = question;
                true
            }
            Msg::GetQuestion(key) => {
                App::refresh_question(ctx, key);
                false
            }
        }
    }

    /// Render the component
    fn view(&self, ctx: &Context<Self>) -> Html {
        let question = &self.question;
        html! {
        <>
            <h1>{ "Questions" }</h1>
            if let Ok(ref question) = question {
                <Question question={question.clone()}/>
            }
            if let Err(ref error) = question {
                <div>
                    <span class="error">{format!("Server Error: {error}")}</span>
                </div>
            }
            <div class="random">
                <button onclick={ctx.link().callback(|_| Msg::GetQuestion(None))}>{"Get Random Question"}</button>
            </div>
            <Finder on_find={ctx.link().callback(Msg::GetQuestion)}/>
        </>
        }
    }
}

fn main() {
    // Render the main app
    yew::Renderer::<App>::new().render();
}
