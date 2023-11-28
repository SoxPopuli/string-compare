import init, { mount } from '../string-compare/pkg/string_compare'
//@ts-ignore
import wasm from '../string-compare/pkg/string_compare_bg.wasm?url'

console.log("test")
let body = document.getElementsByTagName('body')[0]

init(wasm).then(() => {
    console.log('wasm initialized')
    mount(body)
})
