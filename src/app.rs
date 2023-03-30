use chrono::NaiveTime;
use yew::prelude::*;

use crate::components::{Timer, TitleBar};

#[function_component(App)]
pub fn app() -> Html {
    let timer_input_ref = use_node_ref();
    let timer_setting = use_state(|| NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap());

    // タイマー開始ボタンのコールバック
    let start_onclick = {
        let timer_input_ref = timer_input_ref.clone();
        let timer_setting = timer_setting.clone();
        Callback::from(move |_| {
            // タイマーの設定値を取得
            let new_timer_input = timer_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            // 設定値の文字列をNaiveTimeにパース
            let naive_time =
                NaiveTime::parse_from_str(&(String::from("00:") + &new_timer_input), "%H:%M:%S")
                    .unwrap();
            timer_setting.set(naive_time);
        })
    };

    html! {
        <>
            <TitleBar />
            <main class="container">
                <p>{"Until the sound stops :"}</p>
                <Timer time={*timer_setting} />

                <p>
                    <input type="time" ref={timer_input_ref} value="00:00" />
                    <button onclick={start_onclick}> {"Start"} </button>
                </p>
            </main>
        </>
    }
}
