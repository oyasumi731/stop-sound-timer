mod app;
mod components;

use app::App;

fn main() {
    // ロガーの初期化
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
