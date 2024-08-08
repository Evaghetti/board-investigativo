use leptos::*;
use uuid::Uuid;

use crate::{card::Position, entity::CARD_WIDTH_SIZE};

#[derive(Default, Clone)]
pub struct ConnectionLineData {
    id: Uuid,
    id_begin: Uuid,
    id_end: Uuid,
    begin_position: RwSignal<Position>,
    end_position: RwSignal<Position>,
}

impl ConnectionLineData {
    pub fn new(
        id_begin: Uuid,
        id_end: Uuid,
        begin_position: RwSignal<Position>,
        end_position: RwSignal<Position>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            id_begin,
            id_end,
            begin_position,
            end_position,
            ..Default::default()
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn id_begin(&self) -> Uuid {
        self.id_begin.clone()
    }

    pub fn id_end(&self) -> Uuid {
        self.id_end.clone()
    }
}

#[component]
pub fn ConnectionLine(data: ConnectionLineData) -> impl IntoView {
    view! {
      <polygon
          points=move || format!("{},{} {},{}", data.begin_position.get().0 + CARD_WIDTH_SIZE / 2, data.begin_position.get().1, data.end_position.get().0+ CARD_WIDTH_SIZE / 2, data.end_position.get().1)
          style="fill:lime;stroke:purple;stroke-width:5;fill-rule:evenodd;"/>
    }
}
