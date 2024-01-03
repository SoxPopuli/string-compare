mod chartjs;
mod methods;
use methods::Method;

use chartjs::{ChartData, ChartDataset};
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, HtmlElement};

#[component]
fn Charts() -> impl IntoView {
    view! {
        <div class="chart-container">
            <div class="chart-parent">
                <canvas class="chart" id="chart-id"></canvas>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <div class="input-grid">
                <label for="input-1">String 1</label>
                <input id="input-1" type="text" placeholder="String 1"/>
                <label for="input-2">String 2</label>
                <input id="input-2" type="text" placeholder="String 2"/>
            </div>

            <Charts />

            //<ul>
            //    { Method::all()
            //        .map(|x| view! {
            //            <li>
            //                <b>{x.name()}</b>
            //                <i>{x.description()}</i>
            //            </li>
            //        }) 
            //    }
            //</ul>
        </div>
    }
}

#[wasm_bindgen]
pub fn mount(elem: HtmlElement) {
    leptos::mount_to(elem, || view! { <App /> });
}
