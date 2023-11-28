mod methods;

use leptos::*;

use string_compare::App;

pub fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
