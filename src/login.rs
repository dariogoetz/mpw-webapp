use std::error::Error;

use leptos::*;
use mpw::masterkey::MasterKey;

use crate::{storage::EncryptedStorage, LoginData, RwLoginData, RwStorage};

const STORAGE_PASSWORD_SITE: &str = "__storage__";
const STORAGE_PASSWORD_TYPE: &str = "Maximum";
const STORAGE_PASSWORD_COUNTER: i32 = 1;

fn try_login(
    name: &str,
    password: &str,
    storage: &EncryptedStorage,
) -> Result<LoginData, Box<dyn Error>> {
    let masterkey = MasterKey::new_auth(&name, &password);

    let storage_password = masterkey.generate_password(
        STORAGE_PASSWORD_SITE,
        &STORAGE_PASSWORD_TYPE.into(),
        STORAGE_PASSWORD_COUNTER,
    );
    storage.decrypt_sites(name, &storage_password)?;

    let login_data = LoginData {
        name: name.to_string(),
        masterkey,
        storage_password,
    };

    Ok(login_data)
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let login_data = use_context::<RwLoginData>(cx).unwrap().0;
    let store = use_context::<RwStorage>(cx).unwrap().0;

    let name = create_rw_signal(cx, store().last_user);
    let password = create_rw_signal(cx, "".to_string());
    let pw_invalid = create_rw_signal(cx, false);
    let hide_pw = create_rw_signal(cx, true);

    // null password upon login
    create_effect(cx, move |_| {
        if login_data().is_some() {
            password.set("".to_string());
            store.update(|s| s.last_user = login_data().map(|d| d.name).unwrap_or("".to_string()));
        }
    });

    view! { cx,
        <div class="mt-5 text-center">
            <h1 class="display-4 text-light">"Masterpassword App"</h1>
        </div>

        <div class="card col-lg-6 col-md-8 col-12 mt-5 text-center mx-auto">
            <div class="card-body bg-light">
                <form>
                    // Name input field
                    <div class="row my-3 px-3">
                        <div class="input-group">
                            <span class="input-group-text">
                                <i class="fa-solid fa-user"/>
                            </span>
                            <input type="text" class="form-control" placeholder="Full Name"
                                on:input=move |ev| {
                                    name.set(event_target_value(&ev));
                                }
                            prop:value=name
                            />
                        </div>
                    </div>

                    // Password input field
                    <div class="row mb-3 px-3">
                        <div class="input-group">
                            <span class="input-group-text">
                                <i class="fa-solid fa-key"/>
                            </span>
                            <input
                                type=move || if hide_pw() { "password" } else { "text" }
                                placeholder="Password"
                                class=move || {if pw_invalid() {"form-control is-invalid"} else {"form-control"}}
                                on:input=move |ev| {
                                    pw_invalid.set(false);
                                    password.set(event_target_value(&ev));
                                }
                            prop:value=password
                            />
                            <button
                                // toggle password hiding
                                class="btn btn-light btn-outline-secondary"
                                type="button"
                                on:click=move |_| hide_pw.set(!hide_pw())
                            >
                                <i class=move || if hide_pw() {"fa-solid fa-eye"} else {"fa-solid fa-eye-slash"} />
                            </button>
                        </div>
                    </div>


                    // Submit button
                    <div class="row mb-3 px-3">
                        <button type="submit" class="btn btn-secondary text-bg-override"
                            on:click=move |ev| {
                                // stop the page from reloading!
                                ev.prevent_default();

                                if name().len() > 0 {
                                    try_login(&name(), &password(), &store())
                                        .map(|data| login_data.set(Some(data)))
                                        .unwrap_or_else(|_| {
                                            pw_invalid.set(true);
                                    });
                                }
                            }
                        >"Submit"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
