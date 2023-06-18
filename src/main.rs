use yew::prelude::*;


#[function_component]
fn App() -> Html {
    let msg = use_state(|| "");
    let onclick = {
        let msg = msg.clone();
        move |_| {
            msg.set("loading weather info...");
        }
    };

    return html! {
        <div>
            <button {onclick}>{"check weather"}</button>
            <p>{ *msg }</p>
        </div>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}