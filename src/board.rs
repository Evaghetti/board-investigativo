use leptos::*;

use crate::entity::{EntityData, EntityToken};

#[component]
pub fn BoardInvestigativo() -> impl IntoView {
    let (entity_index, set_entity_index) = create_signal(0u64);

    view! {
        <EntityToken data=EntityData::new("Fulano de tal", 20, 10) set_global_entity_index=set_entity_index global_entity_index=entity_index />
        <EntityToken data=EntityData::new("Ciclano", 20, 346) set_global_entity_index=set_entity_index global_entity_index=entity_index />
        <EntityToken data=EntityData::new("Fulaninha da silva silveira", 406, 143) set_global_entity_index=set_entity_index global_entity_index=entity_index />
    }
}
