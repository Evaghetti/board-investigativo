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

#[component]
pub fn BoardInvestigativo() -> impl IntoView {
    let (cards, set_cards) = create_signal(Vec::<CardEntityState>::new());
    let (connection_lines, set_connection_lines) = create_signal(Vec::<ConnectionLineData>::new());

    let (global_index, set_global_index) = create_signal(5u64);
    let (begin_new_line_position, set_begin_new_line_position) =
        create_signal::<Option<RwSignal<Position>>>(None);

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
            .and_then(|s| Some(s.card.position()))
            .and_then(|current_card_position| {
                if let Some(begin) = begin_new_line_position() {
                    let new_line = ConnectionLineData::new(begin, current_card_position);
                    log::info!(
                        "Encerrando a criação de conexão em {:?}",
                        current_card_position.get_untracked()
                    );
                    set_connection_lines.update(move |lines| lines.push(new_line));
                    set_begin_new_line_position(None);
                } else {
                    log::info!(
                        "Começando a criar conexão em {:?}",
                        current_card_position.get_untracked()
                    );
                    set_begin_new_line_position(Some(current_card_position));
                }

                Some(())
            })
            .expect("Não deveria ser possível adicionar uma conexão a um card que não existe");
    };

    let save = move |_: MouseEvent, data: EntityData| {
        set_cards.update(|states| {
            let state = states
                .iter_mut()
                .find(|s| s.entity.id() == data.id())
                .expect("Tentou editar um card que não existe");

            state.editing = false;
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
                                        submit=save/>
                                } >
                                <EntityInfo data=s.entity/>
                             </Show>
                        </Card>
                    }
                }/>
        </main>
    }
}
