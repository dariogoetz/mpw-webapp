use leptos::*;
use mpw::masterkey::MasterKey;

use crate::{storage::EncryptedStorage, GetLoginData, GetUserData, LoginData, SetLoginData};

fn try_login(name: &str, password: &str, storage: &EncryptedStorage) -> Result<LoginData, String> {
    let masterkey = MasterKey::new_auth(&name, &password);

    let storage_password = masterkey.generate_password("__storage__", &"Maximum".into(), 1);
    storage.get_user_sites(name, &storage_password)?;

    let login_data = LoginData {
        name: name.to_string(),
        masterkey,
        storage_password,
    };

    Ok(login_data)
}

#[component]
pub fn Login<F>(cx: Scope, existing_name: F) -> impl IntoView
where
    F: Fn() -> Option<String> + 'static,
{
    let (name, set_name) = create_signal(cx, existing_name().unwrap_or("".to_string()));
    let (password, set_password) = create_signal(cx, "".to_string());
    let (pw_valid, set_pw_valid) = create_signal(cx, true);

    let login_data = use_context::<GetLoginData>(cx)
        .expect("No getter for login data provided")
        .0;
    let set_login_data = use_context::<SetLoginData>(cx)
        .expect("No setter for login data provided")
        .0;

    let user_data = use_context::<GetUserData>(cx)
        .expect("No getter for user data provided")
        .0;

    // null password upon login
    create_effect(cx, move |_| {
        if login_data().is_some() {
            set_password("".to_string());
        }
    });

    view! { cx,
        <div class="card">
            <div class="card-header text-bg-primary"> <h1>"Login"</h1> </div>

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
                        <div class="form-text">"Full name"</div>
                    </div>

                    // Password input field
                    <div class="mb-3">
                        <label class="form-label">"Password"</label>
                        <input
                            type="password"
                            class=move || {if pw_valid() {"form-control"} else {"form-control is-invalid"}}
                            on:input=move |ev| {
                                set_pw_valid(true);
                                set_password(event_target_value(&ev));
                            }
                        prop:value=password
                        />
                    </div>

                    // Submit button
                    <button type="submit" class="btn btn-primary"
                        on:click=move |ev| {
                            // stop the page from reloading!
                            ev.prevent_default();

                            try_login(&name(), &password(), &user_data())
                                .map(|login_data| set_login_data(Some(login_data)))
                                .unwrap_or_else(|_| {
                                    set_pw_valid(false);
                            });
                        }
                    >"Submit"</button>
                </form>
            </div>
        </div>
    }
}
