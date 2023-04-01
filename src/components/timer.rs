use chrono::{Duration, NaiveTime};
use gloo_timers::callback::Timeout;
use stylist::style;
use tauri_sys::tauri;
use wasm_bindgen_futures::spawn_local;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_node_ref, use_state, use_state_eq,
    Callback, Html, Properties,
};

use crate::utils::naive_time_from_zero;

#[derive(Properties, PartialEq)]
pub struct TimerProps {
    pub timer: NaiveTime,
    pub on_decrement_timer: Callback<Duration, ()>,
}

#[function_component(Timer)]
pub(crate) fn timer(props: &TimerProps) -> Html {
    let display_time = use_state_eq(|| props.timer.format("%H:%M:%S").to_string());
    let is_initial_state = use_state_eq(|| true);

    // 引数のログ
    log::info!("Arguments: {}", props.timer.format("%H:%M:%S").to_string());

    {
        // 初期状態を示すフラグの更新
        let is_initial_state = is_initial_state.clone();
        use_effect_with_deps(
            move |timer| {
                if *timer != naive_time_from_zero() {
                    is_initial_state.set(false);
                }
            },
            props.timer,
        )
    }

    {
        // タイマーの残り時間の更新に応じて、表示文字列を更新する
        let display_time = display_time.clone();
        use_effect_with_deps(
            move |timer| {
                let new_display_time = {
                    if *timer >= NaiveTime::from_hms_opt(1, 0, 0).unwrap() {
                        timer.format("%H:%M:%S").to_string()
                    } else {
                        timer.format("%M:%S").to_string()
                    }
                };
                display_time.set(new_display_time.to_string());

                log::info!("Display Time Changed: {}", new_display_time);
            },
            props.timer,
        )
    }

    {
        // タイマーの実行
        let on_decrement_timer = props.on_decrement_timer.clone();
        use_effect_with_deps(
            move |timer| {
                let timer = timer.clone();
                // 1秒ごとに実行
                let timeout = Timeout::new(1_000, move || {
                    let zero = naive_time_from_zero();
                    if timer != zero {
                        // タイマーをデクリメントする
                        log::info!("Timer Decrement.");
                        let duration = Duration::seconds(1);
                        on_decrement_timer.emit(duration);

                        // タイマーが0に変化した場合
                        if timer - duration == zero {
                            // コマンド実行
                            log::info!("Timer Finished!");

                            // 音楽を停止するコマンドをinvokeする
                            spawn_local(
                                async move { tauri::invoke("stop_sound", &{}).await.unwrap() },
                            );
                        }
                    } else {
                        // タイマーが0のときはデクリメントしない
                        log::info!("Timeout Stop.");
                    }
                });

                // クリーンアップ時の処理でTimeoutをクリアする
                || drop(timeout)
            },
            props.timer,
        );
    }

    // スタイル
    let time_style = style! {r#"
        margin-top: 0;
        margin-bottom: 0;
        font-size: 10rem;
    "#}
    .expect("Failed to mount style");
    let flash_style = style! {r#"
        animation: flash 1s linear 5;
        
        @keyframes flash {
            0%,
            100% { opacity: 1; }
            50% { opacity: 0;}
        }
    "#}
    .expect("Failed to mount style");

    html! {
        <>
            <div >
                <p class={if display_time.to_string() == "00:00" && !*is_initial_state{
                    classes!(time_style, flash_style)
                } else {
                    classes!(time_style)
                }}>{display_time.to_string()}</p>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct SettingProps {
    pub on_set_timer: Callback<NaiveTime, ()>,
}

#[function_component(TimerSetting)]
pub(crate) fn timer_setting(props: &SettingProps) -> Html {
    let timer_input = use_state(|| "00:00:00".to_string());
    let timer_input_ref = use_node_ref();

    // タイマー開始ボタンのコールバック
    let start_onclick = {
        let timer_input_ref = timer_input_ref.clone();
        let timer_input = timer_input.clone();
        let on_set_timer = props.on_set_timer.clone();
        Callback::from(move |_| {
            // タイマーの設定値を取得
            let new_timer_input = timer_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            // 入力を初期化
            timer_input.set("00:00:00".to_string());

            // 設定値の文字列をNaiveTimeにパース
            let naive_time = NaiveTime::parse_from_str(&new_timer_input, "%H:%M:%S").unwrap();

            // 設定
            on_set_timer.emit(naive_time);
        })
    };

    html! {
        <>
            <input type="time" ref={timer_input_ref} value={timer_input.to_string()} step="1" />
            <button onclick={start_onclick}> {"Start"} </button>
        </>
    }
}
