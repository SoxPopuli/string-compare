use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone)]
pub struct ChartDataset {
    pub label: String,
    pub data: Vec<f64>,
    #[wasm_bindgen(js_name = "borderWidth")]
    pub border_width: f32,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone)]
pub struct ChartParams {
    #[wasm_bindgen(js_name = "type")]
    pub typ: String,
    pub data: ChartData,
}

#[wasm_bindgen(module = "chart.js")]
extern "C" {
    pub type Chart;

    #[wasm_bindgen(constructor)]
    pub fn new(element: HtmlCanvasElement, params: ChartParams) -> Chart;
}
