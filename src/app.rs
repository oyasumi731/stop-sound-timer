use chrono::Duration;
use yew::prelude::*;

use crate::components::{Timer, TimerSetting, TitleBar};
use crate::utils::naive_time_from_zero;

#[function_component(App)]
pub fn app() -> Html {
    let timer = use_state(|| naive_time_from_zero());

    let set_timer = {
        let timer = timer.clone();
        use_callback(
            move |naive_time, _| {
                timer.set(naive_time);
            },
            (),
        )
    };

    let decrement_timer = {
        let timer = timer.clone();
        use_callback(
            move |duration: Duration, timer| timer.set(*timer.clone() - duration),
            timer,
        )
    };

    html! {
        <>
            <TitleBar />
            <main class="container">
                <p>{"Until the sound stops :"}</p>
                <Timer  timer={*timer}
                        on_decrement_timer={decrement_timer} />
                <p>
                    <TimerSetting on_set_timer={set_timer} />
                </p>
            </main>
        </>
    }
}
