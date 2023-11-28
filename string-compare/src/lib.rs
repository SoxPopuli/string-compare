use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <div>
                <label for="">String 1</label>
                <input type="text" placeholder="String 1"/>
                <label for="">String 2</label>
                <input type="text" placeholder="String 2"/>
            </div>
            <p>hello</p>
        </div>
    }
}

#[wasm_bindgen]
pub fn mount(elem: HtmlElement) {
    leptos::mount_to(elem, || view! { <App /> })
}
