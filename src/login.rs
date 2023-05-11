use leptos::*;
use mpw::masterkey::MasterKey;

use crate::{storage::EncryptedStorage, LoginData, RwLoginData, RwStorage};

const STORAGE_PASSWORD_SITE: &str = "__storage__";
const STORAGE_PASSWORD_TYPE: &str = "Maximum";
const STORAGE_PASSWORD_COUNTER: i32 = 1;

fn try_login(name: &str, password: &str, storage: &EncryptedStorage) -> Result<LoginData, String> {
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

    // null password upon login
    create_effect(cx, move |_| {
        if login_data().is_some() {
            password.set("".to_string());
            store.update(|s| s.last_user = login_data().map(|d| d.name).unwrap_or("".to_string()));
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
                                name.set(event_target_value(&ev));
                            }
                        prop:value=name
                        />
                    </div>

                    // Password input field
                    <div class="mb-3">
                        <label class="form-label">"Password"</label>
                        <input
                            type="password"
                            class=move || {if pw_invalid() {"form-control is-invalid"} else {"form-control"}}
                            on:input=move |ev| {
                                pw_invalid.set(false);
                                password.set(event_target_value(&ev));
                            }
                        prop:value=password
                        />
                    </div>

                    // Submit button
                    <button type="submit" class="btn btn-primary"
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
                </form>
            </div>
        </div>
    }
}
