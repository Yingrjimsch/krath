use leptos::ev::SubmitEvent;
use leptos::mount::mount_to_body;
use leptos::{html, logging, prelude::*};

fn main() {
    mount_to_body(App);
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        <br/>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let values = vec![0, 1, 2];
    let length = 5;
    let counters = (1..=length).map(|idx| signal(idx));
    let counter_buttons = counters
        .map(|(count_2, set_count_2)| {
            view! {
                <li>
                    <button
                        on:click=move |_| *set_count_2.write() += 1
                    >
                        {count_2}
                    </button>
                </li>
            }
        })
        .collect_view();
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);
    let (name_2, set_name_2) = signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name_2.set(value);
    };
    let some_value = RwSignal::new("hello".to_string());
    let (value, set_value) = signal(0i32);
    let (value, set_value) = signal(1);
    let is_odd = move || value.get() % 2 != 0;
    let message = move || is_odd().then(|| "Ding ding ding!");
    let message_2 = move || match value.get() {
        0 => "Zero",
        1 => "One",
        n if is_odd() => "Odd",
        _ => "Even",
    };
    let (value_3, set_value_3) = signal(0);
    let message_3 = move || {
        if value_3.get() > 5 {
            logging::log!("{}: rendering Big", value_3.get());
            "Big"
        } else {
            logging::log!("{}: rendering Small", value_3.get());
            "Small"
        }
    };
    let (value_4, set_value_4) = signal(Ok(0));

    view! {
            <button
                on:click=move |_| {
                    *set_count.write() += 1;
                }
                class:red=move || count.get() % 2 == 1
            >
                "Click me: "
                {count}
            </button>
            <p>
                "Double count: "
                {double_count}
            </p>
            <ProgressBar progress=count/>
            <ProgressBar progress=Signal::derive(double_count)/>
            // this will just render "012"
            <p>{values.clone()}</p>
            // or we can wrap them in <li>
            <ul>
                {values.into_iter()
                    .map(|n| view! { <li>{n}</li>})
                    .collect_view()}
            </ul>
            <ul>{counter_buttons}</ul>
            <button on:click=move |_| {
                set_data.update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
                // log the new value of the signal
                leptos::logging::log!("{:?}", data.get());
            }>
                "Update Values"
            </button>
            // iterate over the rows and display each value
            <For
                each=move || data.get().into_iter().enumerate()
                key=|(_, state)| state.key.clone()
                children=move |(index, _)| {
                    let value = Memo::new(move |_| {
                        data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                    });
                    view! {
                        <p>{value}</p>
                    }
                }
            />
            <input type="text"
            bind:value=(name, set_name)
            />
            <input type="email"
                bind:value=email
            />
            <label>
                "Please send me lots of spam email."
                <input type="checkbox"
                    bind:checked=spam_me
                />
            </label>
            <p>"Name is: " {name}</p>
            <p>"Email is: " {email}</p>
            <Show when=move || spam_me.get()>
                <p>"You’ll receive cool bonus content!"</p>
            </Show>
            <form on:submit=on_submit> // on_submit defined below
            <input type="text"
                value=name_2
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
            </form>
            <p>"Name is: " {name_2}</p>
            <textarea
            prop:value=move || some_value.get()
            on:input:target=move |ev| some_value.set(ev.target().value())
        >
            /* plain-text initial value, does not change if the signal changes */
            {some_value.get_untracked()}
        </textarea>
        <select
        on:change:target=move |ev| {
          set_value.set(ev.target().value().parse().unwrap());
        }
        prop:value=move || value.get().to_string()
      >
        <option value="0">"0"</option>
        <option value="1">"1"</option>
        <option value="2">"2"</option>
      </select>
      // a button that will cycle through the options
      <button on:click=move |_| set_value.update(|n| {
        if *n == 2 {
          *n = 0;
        } else {
          *n += 1;
        }
      })>
        "Next Option"
      </button>
      <p>
          {move || if is_odd() {
              "Odd"
          } else {
              "Even"
          }}
      </p>
      <p>{message}</p>
      <p>{move || is_odd().then(|| "Ding ding ding!")}</p>
      <p>{message_2}</p>
      <Show
      when=move || { value.get() > 5 }
      fallback=move || view! { <p>{message_3}</p> }
    >
        <p>{message_3}</p>
    </Show>
    <main>
        {move || match is_odd() {
            true if value.get() == 1 => {
                // returns HtmlElement<Pre>
                view! { <pre>"Hello One"</pre> }.into_any()
            },
            false if value.get() == 2 => {
                // returns HtmlElement<P>
                view! { <p>"Two"</p> }.into_any()
            }
            // returns HtmlElement<Textarea>
            _ => view! { <textarea>{value.get()}</textarea> }.into_any()
        }}
    </main>
    <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input:target=move |ev| {
                // when input changes, try to parse a number from the input
                set_value_4.set(ev.target().value().parse::<i32>())
            }/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value_4}</strong>
                </p>
            </ErrorBoundary>
        </label>
        }
}
