use ev::MouseEvent;
use leptos::*;
use uuid::Uuid;

use crate::card::{Card, CardData, Position};
use crate::entity::{EntityData, EntityEdit, EntityInfo};
use crate::line::{ConnectionLine, ConnectionLineData};

#[derive(Clone, Copy, Debug)]
struct CardEntityState {
    card: CardData,
    entity: EntityData,
    editing: bool,
}

#[derive(Clone, Copy, Debug)]
struct ConnectionPoint {
    card_id: Uuid,
    position: RwSignal<Position>,
}

#[component]
pub fn BoardInvestigativo() -> impl IntoView {
    let (cards, set_cards) = create_signal(Vec::<CardEntityState>::new());
    let (connection_lines, set_connection_lines) = create_signal(Vec::<ConnectionLineData>::new());

    let (global_index, set_global_index) = create_signal(5u64);
    let (begin_new_line_position, set_begin_new_line_position) =
        create_signal::<Option<ConnectionPoint>>(None);

    let add_new_entity = move |event: MouseEvent| {
        // Apenas ao duplo click
        if event.detail() < 2 {
            return;
        }

        log::info!("Criando card em {} {}", event.x(), event.y());
        let new_card = CardData::new(event.x(), event.y());
        let new_entity = EntityData::new(new_card.id());

        set_cards.update(move |s| {
            s.push(CardEntityState {
                card: new_card,
                entity: new_entity,
                editing: true,
            })
        });
    };

    let add_new_line = move |_event: MouseEvent, id: Uuid| {
        cards()
            .iter()
            .find(|s| s.card.id() == id)
            .and_then(|s| Some(s.card))
            .and_then(|card| {
                if let Some(begin) = begin_new_line_position() {
                    let end = card.position();
                    let new_line =
                        ConnectionLineData::new(begin.card_id, card.id(), begin.position, end);
                    log::info!(
                        "Encerrando a criação de conexão em {:?}",
                        end.get_untracked()
                    );
                    set_connection_lines.update(move |lines| lines.push(new_line));
                    set_begin_new_line_position(None);
                } else {
                    log::info!(
                        "Começando a criar conexão em {:?}",
                        card.position().get_untracked()
                    );
                    set_begin_new_line_position(Some(ConnectionPoint {
                        card_id: card.id(),
                        position: card.position(),
                    }));
                }

                Some(())
            })
            .expect("Não deveria ser possível adicionar uma conexão a um card que não existe");
    };

    let save_edits = move |_: MouseEvent, data: EntityData| {
        set_cards.update(|states| {
            let state = states
                .iter_mut()
                .find(|s| s.entity.id() == data.id())
                .expect("Tentou editar um card que não existe");

            state.editing = false;
        });
    };

    let start_editing = move |data: &EntityData| {
        log::info!("Iniciando editar do {}", data.id());

        set_cards.update(|states| {
            states
                .iter_mut()
                .find(|s| s.entity.id() == data.id())
                .and_then(|s| {
                    s.editing = true;
                    Some(s.editing)
                })
                .expect("Tentou iniciar a edição de um card que não existe");
        });
    };

    let delete_entity = move |_: MouseEvent, data: EntityData| {
        set_cards.update(|states| {
            let index = states
                .iter()
                .position(|s| s.entity.id() == data.id())
                .expect("Tentou deletar um card que não existe");

            set_connection_lines.update(|lines| {
                lines.retain(|l| l.id_begin() != data.id() && l.id_end() != data.id());
            });
            states.remove(index);
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
                each=move || cards.get()
                key=|s| (s.card.id(), s.entity.id(), s.editing)
                children=move |s| {
                    view! {
                        <Card
                            data=s.card
                            global_card_index=global_index
                            set_global_card_index=set_global_index
                            pin_callback=add_new_line>
                            <Show
                                when=move || { !s.editing }
                                fallback=move || view! {
                                    <EntityEdit
                                        data=s.entity
                                        submit_edits_callback=save_edits
                                        delete_entity_callback=delete_entity />
                                } >
                                <EntityInfo
                                    data=s.entity
                                    start_edit_callback=start_editing/>
                             </Show>
                        </Card>
                    }
                }/>
        </main>
    }
}
