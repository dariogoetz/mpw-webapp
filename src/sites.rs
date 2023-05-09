use leptos::*;

use crate::{GetLoginData, GetUserData, SetUserData};

#[component]
pub fn GeneratePasswords(cx: Scope) -> impl IntoView {
    let login_data = use_context::<GetLoginData>(cx)
        .expect("No getter for login data provided")
        .0;
    let login_name = move || login_data().unwrap().name;
    let storage_password = move || login_data().unwrap().storage_password;

    let user_data = use_context::<GetUserData>(cx)
        .expect("No getter for user data provided")
        .0;

    let set_user_data = use_context::<SetUserData>(cx)
        .expect("No setter for user data provided")
        .0;

    let sites = move || {
        user_data()
            .get_user_sites(&login_name(), &storage_password())
            .unwrap_or(Vec::new())
    };

    let save_site = move |name: &str, counter: i32, pw_type: &str| {
        if name.len() > 0 {
            let mut new_user_data = user_data();
            new_user_data.add_site(&login_name(), &storage_password(), name, counter, pw_type);
            set_user_data(new_user_data);
        }
    };

    view! { cx,
        <div class="mb-3 mt-4 p-5 bg-primary text-white rounded">
            <h1 class="display-4">{login_data().unwrap().name}"'s Password Store"</h1>
        </div>

        <NewSite save=save_site/>

        <For
            each=sites
            key=|site| format!("{}_{}_{}", site.site_name, site.counter, site.password_type)
            view=move |cx, site| {
                    view! {cx,
                        <Site
                            name=site.site_name.to_string()
                            counter=site.counter
                            password_type=site.password_type.to_string()
                        />
                    }
                }
        />
    }
}

#[component]
fn NewSite<F>(cx: Scope, save: F) -> impl IntoView
where
    F: Fn(&str, i32, &str) -> () + 'static,
{
    let login_data = use_context::<GetLoginData>(cx)
        .expect("No getter for login data provided")
        .0;
    let masterkey = move || login_data().unwrap().masterkey;

    let (counter, set_counter) = create_signal(cx, 1);
    let (pw_type, set_pw_type) = create_signal(cx, "Maximum".to_string());
    let (hide_pw, set_hide_pw) = create_signal(cx, false);
    let (name, set_name) = create_signal(cx, "".to_string());

    let password = move || {
        if name().len() > 0 {
            masterkey().generate_password(&name(), &pw_type().as_str().into(), counter())
        } else {
            "".to_string()
        }
    };

    let selected = move |selection| {
        if pw_type() == selection {
            Some("selected")
        } else {
            None
        }
    };

    let save_site = move |_| {
        save(&name(), counter(), &pw_type());
    };

    view! { cx,
        <div class="card mb-3">
            <div class="card-header text-bg-primary">
                <div class="row">
                    <h1 class="col-10">"New Site"</h1>
                    <div class="col-2">
                        <button class="btn btn-outline-light float-end" type="button" data-bs-toggle="collapse" data-bs-target="#newSite"><span class="fa-solid fa-ellipsis" /></button>
                        <button class="btn btn-outline-light float-end mx-1" on:click=save_site type="button"><span class="fa-solid fa-download" /></button>
                    </div>
                </div>
            </div>

            <div class="card-body">
                <form>
                    // Name input field
                    <div class="mb-3">
                        <label class="form-label">"Site Name"</label>
                        <input type="text" class="form-control"
                            on:input=move |ev| {
                                set_name(event_target_value(&ev));
                            }
                        prop:value=name
                        />
                    </div>

                    // Password input field
                    <div class="mb-3">
                        <label class="form-label">"Password"</label>
                        <div class="input-group">
                            <input
                                class="form-control"
                                // show password if toggled
                                type={move || if hide_pw() { "password" } else { "text" }}
                                prop:value=password
                                readonly
                            />
                            <button
                                // toggle password hiding
                                class="btn btn-outline-secondary"
                                type="button"
                                on:click=move |_| set_hide_pw(!hide_pw())
                            >
                                <i class={move || if hide_pw() {"fa-solid fa-eye"} else {"fa-solid fa-eye-slash"}} />
                            </button>
                        </div>
                    </div>

                    <div class="row collapse" id="newSite">
                        <div class="col-6">
                            <label>"Password Type"</label>
                            <select
                                class="form-select"
                                on:change=move |ev| set_pw_type(event_target_value(&ev))
                                prop:value=pw_type
                            >
                                <option selected=selected("Maximum") value="Maximum">"Maximum"</option>
                                <option selected=selected("Long") value="Long">"Long"</option>
                                <option selected=selected("Medium") value="Medium">"Medium"</option>
                                <option selected=selected("Short") value="Short">"Short"</option>
                                <option selected=selected("Basic") value="Basic">"Basic"</option>
                                <option selected=selected("PIN") value="PIN">"PIN"</option>
                                <option selected=selected("Name") value="Name">"Name"</option>
                                <option selected=selected("Phrase") value="Phrase">"Phrase"</option>
                            </select>
                        </div>
                        <div class="col-6">

                            <label>"Counter"</label>
                            <input
                                class="form-control"
                                type="number" min="1"
                                on:change=move |ev| set_counter(event_target_value(&ev).parse::<i32>().unwrap_or(1))
                                prop:value=counter
                            />
                        </div>
                    </div>
                </form>
            </div>
        </div>

    }
}

#[component]
fn Site(cx: Scope, name: String, counter: i32, password_type: String) -> impl IntoView {
    let login_data = use_context::<GetLoginData>(cx)
        .expect("No getter for login data provided")
        .0;
    let masterkey = move || login_data().unwrap().masterkey;

    let key = format!("{}_{}_{}", name, counter, password_type);
    let key_target = format!("#{}_{}_{}", name, counter, password_type);

    let (counter, set_counter) = create_signal(cx, counter);
    let (pw_type, set_pw_type) = create_signal(cx, password_type);
    let (hide_pw, set_hide_pw) = create_signal(cx, true);

    let gen_name = name.clone();
    let password =
        move || masterkey().generate_password(&gen_name, &pw_type().as_str().into(), counter());

    let selected = move |selection| {
        if pw_type() == selection {
            Some("selected")
        } else {
            None
        }
    };

    view! { cx,
        <div class="card mb-3">
            <div class="card-header text-bg-secondary">
                <div class="row">
                    <h1 class="col-10">{name.clone()}</h1>
                    <div class="col-2">
                        <button class="btn btn-outline-light float-end" type="button" data-bs-toggle="collapse" data-bs-target={&key_target}><span class="fa-solid fa-ellipsis" /></button>
                    </div>
                </div>
            </div>

            <div class="card-body">
                <form>
                    // Password input field
                    <div class="mb-3">
                        <div class="input-group">
                            <input
                                class="form-control"
                                // show password if toggled
                                type={move || if hide_pw() { "password" } else { "text" }}
                                prop:value=password
                                readonly
                            />
                            <button
                                // toggle password hiding
                                class="btn btn-outline-secondary"
                                type="button"
                                on:click=move |_| set_hide_pw(!hide_pw())
                            >
                                <i class={move || if hide_pw() {"fa-solid fa-eye"} else {"fa-solid fa-eye-slash"}} />
                            </button>
                        </div>
                    </div>

                    <div class="row collapse" id={&key}>
                        <div class="col-6">
                            <label>"Password Type"</label>
                            <select
                                class="form-select"
                                on:change=move |ev| set_pw_type(event_target_value(&ev))
                                prop:value=pw_type
                            >
                                <option selected=selected("Maximum") value="Maximum">"Maximum"</option>
                                <option selected=selected("Long") value="Long">"Long"</option>
                                <option selected=selected("Medium") value="Medium">"Medium"</option>
                                <option selected=selected("Short") value="Short">"Short"</option>
                                <option selected=selected("Basic") value="Basic">"Basic"</option>
                                <option selected=selected("PIN") value="PIN">"PIN"</option>
                                <option selected=selected("Name") value="Name">"Name"</option>
                                <option selected=selected("Phrase") value="Phrase">"Phrase"</option>
                            </select>
                        </div>
                        <div class="col-6">

                            <label>"Counter"</label>
                            <input
                                class="form-control"
                                type="number" min="1"
                                on:change=move |ev| set_counter(event_target_value(&ev).parse::<i32>().unwrap_or(1))
                                prop:value=counter
                            />
                        </div>
                    </div>
                </form>
            </div>
        </div>
    }
}
