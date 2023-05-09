use leptos::*;
use mpw::masterkey::MasterKey;

mod login;
use login::*;

mod storage;
use storage::EncryptedStorage;

mod sites;
use sites::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[derive(Copy, Clone)]
struct GetUserData(ReadSignal<EncryptedStorage>);

#[derive(Copy, Clone)]
struct SetUserData(WriteSignal<EncryptedStorage>);

#[derive(Copy, Clone)]
struct GetLoginData(ReadSignal<Option<LoginData>>);

#[derive(Copy, Clone)]
struct SetLoginData(WriteSignal<Option<LoginData>>);

#[derive(Clone, Debug)]
pub struct LoginData {
    name: String,
    masterkey: MasterKey,
    storage_password: String,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // prepare global state for login data
    let (login_data, set_login_data) = create_signal::<Option<LoginData>>(cx, None);
    provide_context(cx, GetLoginData(login_data));
    provide_context(cx, SetLoginData(set_login_data));

    // prepare global state for browser-local storage
    let (user_data, set_user_data) = create_signal(cx, EncryptedStorage::from_local_storage());
    provide_context(cx, GetUserData(user_data));
    provide_context(cx, SetUserData(set_user_data));
    let existing_name = move || user_data().names().first().map(|name| name.to_string());

    // write database to storage whenever it changes
    create_effect(cx, move |_| {
        user_data().to_local_storage();
    });

    view! { cx,
        <div class="container overflow-hidden">
            <Show
                when=move || login_data().is_some()
                // if no masterpassword is set, yet, show login component
                fallback=move |cx| view! { cx, <Login existing_name=existing_name/>}
            >
                <GeneratePasswords />
            </Show>
        </div>
    }
}
