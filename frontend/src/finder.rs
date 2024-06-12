use crate::*;

/// Prop used for the question id
#[derive(Properties, Clone, PartialEq)]
pub struct FinderProps {
    pub on_find: Callback<Option<String>>,
}

/// Component to render form for getting the question id
#[function_component]
pub fn Finder(props: &FinderProps) -> Html {
    // Get the key
    let key = use_state(|| <Option<String>>::None);

    // Change the key on input from the form
    let change_key = {
        let key = key.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let value = input.value();
            let value = value.trim();
            let value = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
            key.set(value);
        })
    };

    // Clone the props to pass to the form
    let props = props.clone();

    // Return the form to get a question id
    html! { <>
        <div>
            <input type="text" placeholder="Question Id" oninput={change_key}/>
            <button onclick={move |_| props.on_find.emit((*key).clone())}>
                {"Find Question"}
            </button>
        </div>
    </> }
}
