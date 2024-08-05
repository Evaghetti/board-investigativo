use leptos::*;

use crate::entity::{EntityData, EntityToken};
use crate::line::{ConnectionLine, ConnectionLineData};

#[component]
pub fn BoardInvestigativo() -> impl IntoView {
    let raw_entities = vec![
        EntityData::new("Fulano de tal", 20, 10),
        EntityData::new("Ciclano", 20, 346),
        EntityData::new("Fulaninha da silva silveira", 406, 143),
    ];
    let (connection_lines, _set_connection_lines) = create_signal::<Vec<ConnectionLineData>>(vec![
        ConnectionLineData::new(
            raw_entities.get(1).unwrap().position(),
            raw_entities.get(0).unwrap().position(),
        ),
        ConnectionLineData::new(
            raw_entities.get(0).unwrap().position(),
            raw_entities.get(2).unwrap().position(),
        ),
    ]);

    let (entity_index, set_entity_index) = create_signal(5u64);
    let (entities, _set_entities) = create_signal::<Vec<EntityData>>(raw_entities);

    view! {
        <main>
            <svg>
                <For
                    each=move || connection_lines()
                    key=|line| line.id
                    children=move|line| view! {
                        <ConnectionLine data=line/>
                    } />
            </svg>
            <For
                each=move || entities()
                key=|entity| entity.name()
                children=move|entity| view! {
                    <EntityToken data=entity set_global_entity_index=set_entity_index global_entity_index=entity_index />
                } />
        </main>
    }
}
