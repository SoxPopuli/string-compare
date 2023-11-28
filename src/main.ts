import init, { mount, calculate_unnormalized, calculate_normalized, NamedDistance } from '../string-compare/pkg/string_compare'
//@ts-ignore
import wasm from '../string-compare/pkg/string_compare_bg.wasm?url'

import { Chart } from 'chart.js'
import * as charts from './charts'

let unnormalized_empty: NamedDistance[] = []
let normalized_empty: NamedDistance[] = []

console.log("test")
let body = document.getElementsByTagName('body')[0]

function display(chart: Chart, s1: string, s2: string) {
    if(s1 === "" && s2 === "") {
        charts.update_chart(chart, normalized_empty, unnormalized_empty)
    } else {
        let unnormalized = calculate_unnormalized(s1, s2)
        let normalized = calculate_normalized(s1, s2)
        charts.update_chart( chart, normalized, unnormalized )
    }
}

init(wasm).then(() => {
    console.log('wasm initialized')
    mount(body)

    let chart_element = document.getElementById('chart-id')! as HTMLCanvasElement;
    let input_1 = document.getElementById('input-1')! as HTMLInputElement;
    let input_2 = document.getElementById('input-2')! as HTMLInputElement;

    unnormalized_empty = 
        calculate_unnormalized("", "")
        .map(x => { x.distance = 0; return x })
    normalized_empty = 
        calculate_normalized("", "")
        .map(x => { x.distance = 0; return x })
    let chart = charts.display_chart( chart_element, normalized_empty, unnormalized_empty )

    input_1.addEventListener('input', e => {
        let string_1 = (e.target as HTMLInputElement).value
        let string_2 = input_2.value
        //@ts-ignore
        display(chart, string_1, string_2)
    })

    input_2.addEventListener('input', e => {
        let string_1 = input_1.value
        let string_2 = (e.target as HTMLInputElement).value
        //@ts-ignore
        display(chart, string_1, string_2)
    })

})
