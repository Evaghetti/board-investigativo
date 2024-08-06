use ev::MouseEvent;
use leptos::*;
use uuid::Uuid;

pub const CARD_WIDTH_SIZE: i32 = 180;

#[derive(Clone, Debug)]
pub struct EntityData {
    id: Uuid,
    name: String,
    // TODO: comentei só pra tirar os warnings, quando for fazer o editor de entidades recolocar
    // description: String,
    profile_url: String,
    position: RwSignal<(i32, i32)>,
}

impl EntityData {
    pub fn new(name: &str, x: i32, y: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_owned(),
            // TODO: comentei só pra tirar os warnings, quando for fazer o editor de entidades recolocar
            // description: "".to_string(),
            // TODO: Upload de imagem pra não precisar pegar uma da internet
            profile_url:
                "https://i.pinimg.com/originals/f1/0f/f7/f10ff70a7155e5ab666bcdd1b45b726d.jpg"
                    .to_string(),
            position: create_rw_signal((x - CARD_WIDTH_SIZE / 2, y)),
        }
    }

    pub fn position(&self) -> RwSignal<(i32, i32)> {
        self.position
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }
}

#[component]
pub fn EntityToken<PinFn>(
    data: EntityData,
    global_entity_index: ReadSignal<u64>,
    set_global_entity_index: WriteSignal<u64>,
    pin_callback: PinFn,
) -> impl IntoView
where
    PinFn: Fn(MouseEvent, Uuid) + 'static,
{
    let (mouse_offset, set_mouse_offset) = create_signal((0, 0));
    let (dragging, set_dragging) = create_signal(false);
    let (entity_index, set_entity_index) = create_signal(global_entity_index.get_untracked());
    set_global_entity_index(entity_index.get_untracked() + 1);

    let click_in_event = move |event: MouseEvent| {
        // Apenas responder ao click com botão esquerdo
        event.prevent_default();
        event.stop_propagation();
        if event.button() != 0 {
            return;
        }

        let mouse_pos = (event.x(), event.y());
        let new_offset = (
            mouse_pos.0 - data.position.get().0,
            mouse_pos.1 - data.position.get().1,
        );
        set_mouse_offset(new_offset);

        if global_entity_index() != entity_index() {
            let new_index = global_entity_index() + 1;
            set_global_entity_index(new_index);
            set_entity_index(new_index);
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

        data.position
            .set((event.x() - mouse_offset().0, event.y() - mouse_offset().1));
    };
    window_event_listener(ev::mousemove, mouse_move_event);

    let id = data.id();
    view! {
        <article
            class="token"
            style:left=move || format!("{}px", data.position.get().0)
            style:top=move || format!("{}px",  data.position.get().1)
            style:z-index=move || format!("{}", entity_index())
            on:mouseup=click_out_event
            on:mousedown=click_in_event>
            <img id="pin" src="assets/pin.svg" on:click=move |ev| pin_callback(ev, id) />
            <img id="profile" src=data.profile_url/>
            <h2>{data.name}</h2>
        </article>
    }
}
