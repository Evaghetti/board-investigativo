use ev::MouseEvent;
use leptos::*;
use uuid::Uuid;

use crate::entity::{EntityData, EntityToken};
use crate::line::{ConnectionLine, ConnectionLineData};

#[component]
pub fn BoardInvestigativo() -> impl IntoView {
    let (connection_lines, set_connection_lines) = create_signal::<Vec<ConnectionLineData>>(vec![]);
    let (entities, set_entities) = create_signal::<Vec<EntityData>>(vec![]);
    let (entity_index, set_entity_index) = create_signal(5u64);
    let (begin_new_line_position, set_begin_new_line_position) =
        create_signal::<Option<RwSignal<(i32, i32)>>>(None);

    let add_new_entity = move |event: MouseEvent| {
        // Apenas ao duplo click
        if event.detail() < 2 {
            return;
        }

        let new_entity = EntityData::new("Fulano", event.x(), event.y());
        set_entities.update(move |entities| entities.push(new_entity));
    };

    let add_new_line = move |_event: MouseEvent, id: Uuid| {
        entities()
            .iter()
            .find(|e| e.id() == id)
            .and_then(|e| Some(e.position()))
            .and_then(|p| {
                if let Some(begin) = begin_new_line_position() {
                    let new_line = ConnectionLineData::new(begin, p);
                    set_connection_lines.update(move |lines| lines.push(new_line));
                    set_begin_new_line_position(None);
                } else {
                    set_begin_new_line_position(Some(p));
                }

                Some(())
            });
    };

    view! {
        <main on:click=add_new_entity>
            <svg>
                <For
                    each=move || connection_lines()
                    key=|line| line.id()
                    children=move|line| view! {
                        <ConnectionLine data=line/>
                    } />
            </svg>
            <For
                each=move || entities()
                key=|entity| entity.id()
                children=move|entity| view! {
                    <EntityToken
                        data=entity
                        set_global_entity_index=set_entity_index
                        global_entity_index=entity_index
                        pin_callback=add_new_line />
                } />
            <p>{move || entities().len()}</p>
        </main>
    }
}
