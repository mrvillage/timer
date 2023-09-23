use leptos::*;
use leptos_dom::IntoView;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_use::{use_interval_fn_with_options, utils::Pausable, UseIntervalFnOptions};
use mrvillage_ui::{Button, ButtonColor, NumberInput};

#[component]
pub fn Test(#[prop(into, optional)] value: MaybeSignal<i128>) -> impl IntoView {
    view! {
        <input type="text" inputmode="numeric" prop:value=value />
    }
}
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (duration, set_duration) = create_signal::<i128>(0);
    let duration_secs = Signal::derive(move || duration() / 1000);
    let (remaining, set_remaining) = create_signal::<i128>(0);
    let (running_duration, set_running_duration) = create_signal::<i128>(0);
    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn_with_options(
        move || {
            let rem = remaining.get_untracked();
            if rem <= 100 {
                set_remaining(0);
            } else if rem != 0 {
                set_remaining(rem - 100);
            }
        },
        100,
        UseIntervalFnOptions {
            immediate: false,
            ..Default::default()
        },
    );
    let pause2 = pause.clone();
    let pause3 = pause.clone();
    create_effect(move |_| {
        if remaining() == 0 && is_active() {
            pause3();
        }
    });
    // <Stylesheet id="leptos" href="/pkg/timer.css" />
    // <Stylesheet id="mrvillage-ui" href="/mrvillage-ui.css" />
        // <Title text="Timer" />

                // {
                //     println!("SHOWING");
                //     let pause = pause.clone();
                //     view! {
                //         <Button class="mx-auto" color=ButtonColor::Red on:click=move |_| {
                //             pause();
                //             set_remaining(duration());
                //             set_running_duration(duration());
                //         }>
                //             Reset
                //         </Button>
                //     }
                // }
    view! {
        <div class="dark">
            <main class="text-center pt-4 min-h-[100vh] w-full !max-w-full mu-main-bg mu-prose">
                <div>
                    <NumberInput<i128> value=duration_secs on:input=move |ev| {
                        let value = event_target_value(&ev);
                        if value.is_empty() {
                            set_duration(0);
                            return;
                        }
                        match event_target_value(&ev).parse::<i128>() {
                            Ok(v) => set_duration(v * 1000),
                            Err(_) => set_duration.update(|_| ()),
                        }
                    }/>
                </div>
                <Show
                    when=move || false
                    fallback=move || ()
                >
                {}
                </Show>
                <Button class="mx-auto" color=ButtonColor::Indigo on:click=move |_| {
                    if is_active() {
                        pause2();
                    } else {
                        if duration() <= 0 {
                            return;
                        }
                        if duration() != running_duration() || remaining() <= 0 {
                            set_remaining(duration());
                            set_running_duration(duration());
                        }
                        resume();
                    }
                }>
                    {move || if is_active() {"Stop"} else {"Start"}}
                </Button>
                <div>
                    {move || format!("{:.2}", remaining() as f64 / 1000.0)}
                </div>
            </main>
        </div>
    }
}
