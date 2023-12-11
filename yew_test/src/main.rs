use yew::{function_component, html, Html};
mod components;
use components::grid::Grid;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <p>{"Test"}</p>
        <Grid />
    </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
