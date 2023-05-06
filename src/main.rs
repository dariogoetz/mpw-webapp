use leptos::*;

use mpw::masterkey::MasterKey;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}


#[derive(Clone, Debug)]
struct UserData {
    name: String,
    masterkey: MasterKey,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // provide a masterpassword signal to place into context (global state)
    let (userdata, set_userdata) = create_signal::<Option<UserData>>(cx, None);

    provide_context(cx, set_userdata);

    view! { cx,
        <div class="container overflow-hidden">
            <Show
                when=move || userdata().is_some()
                // if no masterpassword is set, yet, show login component
                fallback=|cx| view! { cx, <Login />}
            >
            // TODO: provide password generation component
            "Logged in!"
            </Show>
            <h1>{move || format!("Masterpassword: {:?}", userdata())}</h1>
        </div>
    }
}

fn mpw_generator(name: String, password: String) -> UserData {
    let masterkey = MasterKey::new_auth(&name, &password);

    let userdata = UserData {
        name: name.clone(),
        masterkey,
    };

    userdata
}

#[component]
fn Login(cx: Scope) -> impl IntoView {
    // TODO: get users (with last used info) from local storage
    let users = vec!["Dario Götz", "dario", "allyn", "mattis"];

    view! { cx,
        <div class="row">
        // Existing users
        {users
        .into_iter()
        .map(|user| {
            view! { cx,
                <div class="col-12 p-3">
                    <LoginExistingUser name=user.to_string() mpw_generator=mpw_generator/>
                </div>}
        })
        .collect::<Vec<_>>()}
        </div>

        <div class="row">
        // New User
        <div class="col-12">
            <LoginNewUser mpw_generator=mpw_generator/>
        </div></div>
    }
}

#[component]
fn LoginExistingUser<F>(cx: Scope, name: String, mpw_generator: F) -> impl IntoView
where
    F: Fn(String, String) -> UserData + 'static,
{
    let (password, set_password) = create_signal(cx, "".to_string());

    // set masterpassword in global context
    let set_userdata =
        use_context::<WriteSignal<Option<UserData>>>(cx).expect("No setter for user data provided");

    view! { cx,
        <div class="card">
            <div class="card-header text-bg-secondary"> <h1>{name.clone()}</h1> </div>

            <div class="card-body">
                <form>
                    // Password input field
                    <div class="mb-3">
                        <label class="form-label">"Password"</label>
                        <input type="password" class="form-control"
                            on:input=move |ev| {
                                set_password(event_target_value(&ev));
                            }
                        prop:value=password
                        />
                    </div>

                    <button type="submit" class="btn btn-secondary"
                        on:click=move |ev| {
                            // stop the page from reloading!
                            ev.prevent_default();

                            let userdata = mpw_generator(name.clone(), password());
                            set_userdata(Some(userdata));
                        }
                    >"Submit"</button>
                </form>
            </div>
        </div>
    }
}

#[component]
fn LoginNewUser<F>(cx: Scope, mpw_generator: F) -> impl IntoView
where
    F: Fn(String, String) -> UserData + 'static,
{
    let (name, set_name) = create_signal(cx, "".to_string());
    let (password, set_password) = create_signal(cx, "".to_string());

    // set masterpassword in global context
    let set_userdata = use_context::<WriteSignal<Option<UserData>>>(cx)
        .expect("No setter for masterpassword provided");

    view! { cx,
        <div class="card">
            <div class="card-header text-bg-primary"> <h1>"New User"</h1> </div>

            <div class="card-body">
                <form>
                    // Name input field
                    <div class="mb-3">
                        <label class="form-label">"Name"</label>
                        <input type="text" class="form-control"
                            on:input=move |ev| {
                                set_name(event_target_value(&ev));
                            }
                        prop:value=name
                        />
                        <div id="emailHelp" class="form-text">"Full name"</div>
                    </div>

                    // Password input field
                    <div class="mb-3">
                        <label class="form-label">"Password"</label>
                        <input type="password" class="form-control"
                            on:input=move |ev| {
                                set_password(event_target_value(&ev));
                            }
                        prop:value=password
                        />
                    </div>

                    <button type="submit" class="btn btn-primary"
                        on:click=move |ev| {
                            // stop the page from reloading!
                            ev.prevent_default();

                            let userdata = mpw_generator(name(), password());
                            set_userdata(Some(userdata));
                        }
                    >"Submit"</button>
                </form>
            </div>
        </div>
    }
}
