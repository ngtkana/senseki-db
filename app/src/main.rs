use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"スマブラSP 戦績管理"</h1>
            <p>"Hello, Leptos!"</p>
        </div>
    }
}
