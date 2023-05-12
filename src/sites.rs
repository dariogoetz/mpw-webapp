use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{RwLoginData, RwStorage};

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

    let store = use_context::<RwStorage>(cx).unwrap().0;

    let filter = create_rw_signal(cx, "".to_string());
    let sites = move || {
        store()
            .decrypt_sites(&login_name(), &storage_password())
            .unwrap_or(Vec::new())
            .into_iter()
            .filter(|s| s.site_name.contains(&filter()))
            .collect::<Vec<_>>()
    };

    view! { cx,
        <div class="my-5 text-center">
            <h1 class="display-4 text-light">{login_data().unwrap().name}"'s Password Store"</h1>
        </div>

        <SitePassword site=Signal::derive(cx, move || None)/>

        <hr />
        <div class="row justify-content-end mb-1">
            <div class="col-3">
                <div class="input-group">
                    <span class="input-group-text">
                        <i class="fa-solid fa-filter"/>
                    </span>
                    <input type="text" class="form-control" placeholder="Filter..."
                        on:input=move |ev| {
                            filter.set(event_target_value(&ev));
                        }
                        prop:value=filter
                    />
                </div>
            </div>
        </div>

        <div class="row">
            {move || sites().into_iter().map(|site| {
                view! {cx,
                    <div class="col-lg-6"><SitePassword site=Signal::derive(cx, move || Some(site.clone())) /></div>
                }
            })
            .collect::<Vec<_>>()}
            // For some reason, the <For /> construct looses the sorting after filtering
            // we therefore use the (less efficient) version above

            // <For
            //     each=sites
            //     key=|site| site.site_name.to_string()
            //     view=move |cx, site| {
            //         view! {cx,
            //             <div class="col-lg-6"><SitePassword site=Signal::derive(cx, move || Some(site.clone())) /></div>
            //         }
            //     }
            // />
        </div>
    }
}

#[component]
fn SitePassword(cx: Scope, site: Signal<Option<Site>>) -> impl IntoView {
    // from global context
    let login_data = use_context::<RwLoginData>(cx).unwrap().0;
    let login_name = move || login_data().unwrap().name;
    let masterkey = move || login_data().unwrap().masterkey;
    let storage_password = move || login_data().unwrap().storage_password;

    let store = use_context::<RwStorage>(cx).unwrap().0;

    // signals
    let site_name = create_rw_signal(cx, site().map(|s| s.site_name).unwrap_or("".to_string()));
    let counter = create_rw_signal(cx, site().map(|s| s.counter).unwrap_or(1));
    let pw_type = create_rw_signal(
        cx,
        site()
            .map(|s| s.password_type)
            .unwrap_or("Maximum".to_string()),
    );
    let hide_pw = create_rw_signal(cx, site().is_some());

    // derived signals

    let title = || {
        site()
            .map(|_| site_name())
            .unwrap_or("New Site".to_string())
    };
    let is_selected = move |selection| (pw_type() == selection).then(|| "selected");

    let add_site = move || {
        if site_name().len() > 0 {
            store.update(|data| {
                data.add_site(
                    &login_name(),
                    &storage_password(),
                    &site_name(),
                    counter(),
                    &pw_type(),
                )
            });
        }
    };

    let update_site = move || {
        store.update(|data| {
            data.update_site(
                &login_name(),
                &storage_password(),
                &site_name(),
                counter(),
                &pw_type(),
            )
        });
    };

    let delete_site = move |_ev| {
        store.update(|data| data.delete_site(&login_name(), &storage_password(), &site_name()));
    };

    let save_site = move || {
        if site().is_none() {
            add_site();
        } else {
            update_site();
        }
    };

    let password = move || {
        if site_name().len() > 0 {
            // save_site(&site_name(), counter(), &pw_type()); // generates infinite loop...?
            store();
            masterkey().generate_password(&site_name(), &pw_type().as_str().into(), counter())
        } else {
            "".to_string()
        }
    };

    let save_on_click = move |ev: ev::MouseEvent| {
        ev.prevent_default();

        save_site();

        site_name.set("".to_string());
        counter.set(1);
        pw_type.set("Maximum".to_string());
    };

    view! { cx,
        <form>
            <div class="card mb-3 border-dark">
                <div class=move || if site().is_none() {"card-header text-bg-secondary text-bg-override"} else {"card-header"}>
                    <div class="row">
                        <span class="col-8 fs-4">{title()}</span>
                        <div class="col-4">
                            // Collapse edit fields button
                            <button class="btn btn-light btn-outline-secondary float-end" type="button" data-bs-toggle="collapse" data-bs-target=format!("#{}Collapse", site_name())>
                                <i class="fa-solid fa-ellipsis" />
                            </button>

                            {if site().is_none() {
                                view! { cx,
                                    // save site password
                                    <button class="btn btn-light btn-outline-secondary float-end" on:click=save_on_click type="submit">
                                        <i class="fa-solid fa-download" />
                                    </button>
                                }
                            } else {
                                view! {cx,
                                    // Delete password button
                                    <button class="btn btn-light btn-outline-danger float-end" on:click=delete_site type="button">
                                        <i class="fa-solid fa-trash-can" />
                                    </button>
                                }
                            }}

                        </div>
                    </div>
                </div>

                <div class="card-body text-bg-light">
                    {if site().is_none() {
                        view! { cx,
                            // Name input field
                            <div class="input-group mb-3">
                                <span class="input-group-text">
                                    <i class="fa-solid fa-pen"/>
                                </span>
                                <input type="text" class="form-control" placeholder="Site Name"
                                    on:input=move |ev| {
                                        site_name.set(event_target_value(&ev));
                                    }
                                    prop:value=site_name
                                />
                            </div>
                        }.into_any()
                    } else {view! { cx, <div />}.into_any()}}

                    // Password input field
                    <div class="input-group mb-3">
                        <input
                            class="form-control text-bg-secondary text-bg-override text-center"
                            // show password if toggled
                            type=move || if hide_pw() { "password" } else { "text" }
                            prop:value=password
                            readonly
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

                    // Password Settings
                    <div class="row collapse" id=format!("{}Collapse", site_name())>
                        <div class="col-6">

                            // Password Type Select
                            <label>"Password Type"</label>
                            <select
                                class="form-select"
                                on:change=move |ev| {
                                    pw_type.set(event_target_value(&ev));
                                    if site().is_some() {
                                        save_site();
                                    }
                                }
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
                                on:change=move |ev| {
                                    counter.set(event_target_value(&ev).parse::<i32>().unwrap_or(1));
                                    if site().is_some() {
                                        save_site();
                                    }
                                }
                                prop:value=counter
                            />
                        </div>
                    </div>
                </div>
            </div>
        </form>
    }
}
