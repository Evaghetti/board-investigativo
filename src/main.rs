use leptos::*;

mod entity;

use entity::{EntityData, EntityToken};

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <div>
                <EntityToken data=EntityData::new("Fulano de tal")/>
                <EntityToken data=EntityData::new("Ciclaninho")/>
                <EntityToken data=EntityData::new("Fulana")/>
            </div>
        }
    })
}
