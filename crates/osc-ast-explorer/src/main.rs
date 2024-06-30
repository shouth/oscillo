use osc_parser::parser::to_osc_syntax_string;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let ast = use_state(|| to_osc_syntax_string(""));

    let oninput = {
        let ast = ast.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
            let text = target.value();

            let syntax = to_osc_syntax_string(&text);
            ast.set(syntax);
        })
    };

    html! {
        <div>
            <h1>{"OSC AST Explorer"}</h1>
            <div style="display: flex; flex-flow: row; justify-content: space-evenly;">
                <textarea rows={10} cols={80} {oninput} style="width: 45%" />
                <pre style="width: 45%">{(*ast).clone()}</pre>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
