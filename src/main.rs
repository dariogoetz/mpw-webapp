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
struct RwStorage(RwSignal<EncryptedStorage>);

#[derive(Copy, Clone)]
struct RwLoginData(RwSignal<Option<LoginData>>);

#[derive(Clone, Debug)]
pub struct LoginData {
    name: String,
    masterkey: MasterKey,
    storage_password: String,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // prepare global state for login data
    let login_data = create_rw_signal::<Option<LoginData>>(cx, None);
    provide_context(cx, RwLoginData(login_data));

    // prepare global state for browser-local storage
    let store = create_rw_signal(cx, EncryptedStorage::from_local_storage());
    provide_context(cx, RwStorage(store));

    // write database to storage whenever it changes
    create_effect(cx, move |_| {
        store().to_local_storage();
    });

    view! { cx,
        <div class="container overflow-hidden">
            <Show
                when=move || login_data().is_some()
                // if no masterpassword is set, yet, show login component
                fallback=move |cx| view! { cx, <Login />}
            >
                <Sites />
            </Show>
        </div>
    }
}
