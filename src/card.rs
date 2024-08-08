use ev::MouseEvent;
use leptos::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Default)]
pub struct Position(pub i32, pub i32);

#[derive(Clone, Copy, Debug)]
pub struct CardData {
    id: Uuid,
    position: RwSignal<Position>,
}

impl CardData {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            position: create_rw_signal(Position(x, y)),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn position(&self) -> RwSignal<Position> {
        self.position
    }
}

#[component]
pub fn Card<PinFn>(
    data: CardData,
    global_card_index: ReadSignal<u64>,
    set_global_card_index: WriteSignal<u64>,
    pin_callback: PinFn,
    children: Children,
) -> impl IntoView
where
    PinFn: Fn(MouseEvent, Uuid) + 'static,
{
    let (mouse_offset, set_mouse_offset) = create_signal(Position::default());
    let (dragging, set_dragging) = create_signal(false);
    let (card_index, set_card_index) = create_signal(global_card_index.get_untracked());
    set_global_card_index(card_index.get_untracked() + 1);

    let click_in_event = move |event: MouseEvent| {
        // Apenas responder ao click com botão esquerdo
        event.stop_propagation();
        if event.button() != 0 {
            return;
        }

        let mouse_pos = (event.x(), event.y());
        let new_offset = Position(
            mouse_pos.0 - data.position.get().0,
            mouse_pos.1 - data.position.get().1,
        );
        set_mouse_offset(new_offset);

        if global_card_index() != card_index() {
            let new_index = global_card_index() + 1;
            set_global_card_index(new_index);
            set_card_index(new_index);
        }

        set_dragging(true);
    };

    let click_out_event = move |event: MouseEvent| {
        event.prevent_default();
        if !dragging() {
            return;
        }

        set_dragging(false);
    };

    let mouse_move_event = move |event: MouseEvent| {
        if !dragging() {
            return;
        }

        data.position.set(Position(
            event.x() - mouse_offset().0,
            event.y() - mouse_offset().1,
        ));
    };
    let handle = window_event_listener(ev::mousemove, mouse_move_event);
    on_cleanup(|| handle.remove());

    let id = data.id;
    view! {
        <article
            class="token"
            style:left=move || format!("{}px", data.position.get().0)
            style:top=move || format!("{}px",  data.position.get().1)
            style:z-index=move || format!("{}", card_index())
            on:mouseup=click_out_event
            on:mousedown=click_in_event>
            <img id="pin" src="assets/pin.svg" on:click=move |ev| pin_callback(ev, id) />
            {children()}
        </article>
    }
}
