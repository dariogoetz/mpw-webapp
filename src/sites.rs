use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{RwLoginData, RwUserData};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Site {
    pub site_name: String,
    pub counter: i32,
    pub password_type: String,
}

#[component]
pub fn Sites(cx: Scope) -> impl IntoView {
    let login_data = use_context::<RwLoginData>(cx).unwrap().0;
    let login_name = move || login_data().unwrap().name;
    let storage_password = move || login_data().unwrap().storage_password;

    let user_data = use_context::<RwUserData>(cx).unwrap().0;

    let sites = move || {
        user_data()
            .decrypt_sites(&login_name(), &storage_password())
            .unwrap_or(Vec::new())
    };

    view! { cx,
        <div class="mb-3 mt-4 p-5 bg-primary text-white rounded">
            <h1 class="display-4">{login_data().unwrap().name}"'s Password Store"</h1>
        </div>

        <SitePassword site=None/>

        <For
            each=sites
            key=|site| site.site_name.to_string()
            view=move |cx, site| {
                    view! {cx,
                        <SitePassword site=Some(site) />
                    }
                }
        />
    }
}

#[component]
fn SitePassword(cx: Scope, site: Option<Site>) -> impl IntoView {
    let site = move || site.clone();

    // from global context
    let login_data = use_context::<RwLoginData>(cx)
        .expect("No getter for login data provided")
        .0;
    let login_name = move || login_data().unwrap().name;
    let masterkey = move || login_data().unwrap().masterkey;
    let storage_password = move || login_data().unwrap().storage_password;

    let user_data = use_context::<RwUserData>(cx).unwrap().0;

    // signals
    let counter = create_rw_signal(cx, site().map(|s| s.counter).unwrap_or(1));
    let pw_type = create_rw_signal(
        cx,
        site()
            .map(|s| s.password_type)
            .unwrap_or("Maximum".to_string()),
    );
    let site_name = create_rw_signal(cx, site().map(|s| s.site_name).unwrap_or("".to_string()));
    let hide_pw = create_rw_signal(cx, site().is_some());

    // derived signals
    let title = || {
        site()
            .map(|_| site_name())
            .unwrap_or("New Site".to_string())
    };
    let password = move || {
        if site_name().len() > 0 {
            // TODO: save, once site update is supported by storage
            //save(&site_name(), counter(), &pw_type());
            masterkey().generate_password(&site_name(), &pw_type().as_str().into(), counter())
        } else {
            "".to_string()
        }
    };

    let is_selected = move |selection| (pw_type() == selection).then(|| "selected");

    let save_site = move |ev: ev::MouseEvent| {
        ev.prevent_default();

        if site_name().len() > 0 {
            user_data.update(|data| {
                data.add_site(
                    &login_name(),
                    &storage_password(),
                    &site_name(),
                    counter(),
                    &pw_type(),
                )
            });
            site_name.set("".to_string());
        }
    };

    let delete_site = move |_ev| {
        // TODO
        log!("Deleting site {}", site_name());
    };

    view! { cx,
        <form>
            <div class="card mb-3">
                <div class="card-header text-bg-secondary">
                    <div class="row">
                        <h1 class="col-9">{title()}</h1>
                        <div class="col-3">
                            // Collapse edit fields button
                            <button class="btn btn-outline-light float-end" type="button" data-bs-toggle="collapse" data-bs-target=format!("#{}Collapse", site_name())>
                                <i class="fa-solid fa-ellipsis" />
                            </button>

                            {if site().is_none() {
                                view! { cx,
                                    // save site password
                                    <button class="btn btn-outline-light float-end" on:click=save_site type="submit">
                                        <i class="fa-solid fa-download" />
                                    </button>
                                }
                            } else {
                                view! {cx,
                                    // Delete password button
                                    <button class="btn btn-outline-danger float-end" on:click=delete_site type="button">
                                        <i class="fa-solid fa-trash-can" />
                                    </button>
                                }
                            }}

                        </div>
                    </div>
                </div>

                <div class="card-body">
                    {if site().is_none() {
                        view! { cx,
                            // Name input field
                            <div class="mb-3">
                                <label class="form-label">"Site Name"</label>
                                <input type="text" class="form-control"
                                    on:input=move |ev| {
                                        site_name.set(event_target_value(&ev));
                                    }
                                    prop:value=site_name
                                />
                            </div>
                        }.into_any()
                    } else {view! { cx, <div />}.into_any()}}

                    // Password input field
                    <div class="mb-3">
                        <div class="input-group">
                            <input
                                class="form-control"
                                // show password if toggled
                                type=move || if hide_pw() { "password" } else { "text" }
                                prop:value=password
                                readonly
                            />
                            <button
                                // toggle password hiding
                                class="btn btn-outline-secondary"
                                type="button"
                                on:click=move |_| hide_pw.set(!hide_pw())
                            >
                                <i class=move || if hide_pw() {"fa-solid fa-eye"} else {"fa-solid fa-eye-slash"} />
                            </button>
                        </div>
                    </div>

                    // Password Settings
                    <div class="row collapse" id=format!("{}Collapse", site_name())>
                        <div class="col-6">

                            // Password Type Select
                            <label>"Password Type"</label>
                            <select
                                class="form-select"
                                on:change=move |ev| pw_type.set(event_target_value(&ev))
                                prop:value=pw_type
                            >
                                <option selected=is_selected("Maximum") value="Maximum">"Maximum"</option>
                                <option selected=is_selected("Long") value="Long">"Long"</option>
                                <option selected=is_selected("Medium") value="Medium">"Medium"</option>
                                <option selected=is_selected("Short") value="Short">"Short"</option>
                                <option selected=is_selected("Basic") value="Basic">"Basic"</option>
                                <option selected=is_selected("PIN") value="PIN">"PIN"</option>
                                <option selected=is_selected("Name") value="Name">"Name"</option>
                                <option selected=is_selected("Phrase") value="Phrase">"Phrase"</option>
                            </select>
                        </div>

                        <div class="col-6">

                            // Password Counter
                            <label>"Counter"</label>
                            <input
                                class="form-control"
                                type="number" min="1"
                                on:change=move |ev| counter.set(event_target_value(&ev).parse::<i32>().unwrap_or(1))
                                prop:value=counter
                            />
                        </div>
                    </div>
                </div>
            </div>
        </form>
    }
}
