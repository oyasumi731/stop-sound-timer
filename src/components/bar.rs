use stylist::style;
use tauri_sys::window;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, Callback, Html};

#[function_component(TitleBar)]
pub(crate) fn titlebar() -> Html {
    // 最小化ボタンのハンドラ
    let minimize_onclick = Callback::from(move |_| {
        spawn_local(async move {
            window::current_window()
                .minimize()
                .await
                .expect("Failed to minimize.");
        });
    });
    // トグルボタンのハンドラ
    let toggle_onclick = Callback::from(move |_| {
        spawn_local(async move {
            window::current_window()
                .toggle_maximize()
                .await
                .expect("Failed to maximize.");
        });
    });
    // 閉じるボタンのハンドラ
    let close_onclick = Callback::from(move |_| {
        spawn_local(async move {
            window::current_window()
                .close()
                .await
                .expect("Failed to close.");
        });
    });

    // スタイル
    let titlebar_style = style! {r#"
        height: 40px;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        top: 0;
        left: 0;
        right: 0;
    "#}
    .expect("Failed to mount style");
    let button_style = style! {r#"
        background: #329ea3;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 40px;
        height: 40px;

        :hover{
            background: #5bbec3;   
        }
    "#}
    .expect("Failed to mount style");
    let closebutton_style = style! {r#"
        :hover {
            background: #D31323;  
        }
    "#}
    .expect("Failed to mount style");

    html! {
        <>
            <div data-tauri-drag-region="" class={titlebar_style}>
                <div class={button_style.clone()} onclick={minimize_onclick}>
                    <img
                    src="https://api.iconify.design/mdi:window-minimize.svg"
                    alt="minimize"
                    />
                </div>
                <div class={button_style.clone()} onclick={toggle_onclick}>
                    <img
                    src="https://api.iconify.design/mdi:window-maximize.svg"
                    alt="maximize"
                    />
                </div>
                <div class={classes!{button_style.clone(), closebutton_style}} onclick={close_onclick}>
                    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
                </div>
            </div>
        </>
    }
}
