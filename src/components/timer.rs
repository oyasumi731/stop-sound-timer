use chrono::{Duration, NaiveTime};
use gloo_timers::callback::Timeout;
use stylist::style;
use tauri_sys::tauri;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub time: NaiveTime,
}

#[function_component(Timer)]
pub(crate) fn timer(props: &Props) -> Html {
    let local_time = use_state(|| props.time);
    let display_time = use_state(|| props.time.format("%M:%S").to_string());

    // 引数のログ
    log::info!("Arguments: {}", props.time.format("%M:%S").to_string());

    {
        // タイマーの設定
        let local_time = local_time.clone();
        let new_time = props.time.clone();
        use_effect_with_deps(
            move |_| {
                local_time.set(new_time);
            },
            props.time,
        );
    }

    {
        // 内部時間の更新に応じて、表示文字列を更新する
        let local_time = local_time.clone();
        let display_time = display_time.clone();
        use_effect_with_deps(
            move |local_time| {
                let new_display_time = local_time.format("%M:%S").to_string();
                display_time.set(new_display_time.clone());

                log::info!("Display Time Changed: {}", new_display_time);
            },
            local_time,
        )
    }

    {
        // タイマーの実行
        let local_time = local_time.clone();
        use_effect_with_deps(
            move |local_time| {
                let local_time = local_time.clone();

                // 1秒ごとに実行
                let timeout = Timeout::new(1_000, move || {
                    let zero = naive_time_from_zero();
                    let new_time = {
                        if *local_time.clone() != zero {
                            log::info!("Timer Decrement.");

                            *local_time.clone() - Duration::seconds(1)
                        } else {
                            log::info!("Timeout Stop.");

                            // 内部時間が0になったらデクリメントを停止する
                            *local_time.clone()
                        }
                    };
                    local_time.set(new_time);

                    // タイマーが0になったとき
                    if new_time == zero && new_time < *local_time {
                        // コマンド実行
                        log::info!("Timer Finished!");

                        // 音楽を停止するコマンドをinvokeする
                        spawn_local(async move { tauri::invoke("stop_sound", &{}).await.unwrap() });
                    }
                });

                // クリーンアップ時の処理でTimeoutをクリアする
                || drop(timeout)
            },
            local_time,
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
                <p class={if &*display_time.clone() == "00:00"{
                    classes!(time_style, flash_style)
                } else {
                    classes!(time_style)
                }}>{&*display_time.clone()}</p>
            </div>
        </>
    }
}

/// 0時0分0秒のNaiveTimeを返します
fn naive_time_from_zero() -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
}
