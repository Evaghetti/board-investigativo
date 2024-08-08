use ev::MouseEvent;
use leptos::*;
use uuid::Uuid;

pub const CARD_WIDTH_SIZE: i32 = 180;

#[derive(Clone, Copy, Debug)]
pub struct EntityData {
    id: Uuid,
    name: RwSignal<String>,
    description: RwSignal<String>,
    profile_url: RwSignal<String>,
}

impl EntityData {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            name: create_rw_signal("".to_string()),
            description: create_rw_signal("".to_string()),
            // TODO: Upload de imagem pra não precisar pegar uma da internet
            profile_url: create_rw_signal(
                "https://i.pinimg.com/originals/f1/0f/f7/f10ff70a7155e5ab666bcdd1b45b726d.jpg"
                    .to_string(),
            ),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }
}

#[component]
pub fn EntityInfo(data: EntityData) -> impl IntoView {
    view! {
        <img id="profile" src=data.profile_url.get()/>
        <h2>{data.name.get()}</h2>
    }
}

#[component]
pub fn EntityEdit<SubmitFn>(data: EntityData, submit: SubmitFn) -> impl IntoView
where
    SubmitFn: Fn(MouseEvent, EntityData) + 'static,
{
    log::info!("Editando entidade {}", data.id());
    view! {
        <input
            prop:value=move || data.name.get()
            on:input=move |ev| data.name.set(event_target_value(&ev)) />
        <textarea
            prop:value=move || data.description.get()
            on:input=move |ev| data.description.set(event_target_value(&ev))>
            {data.description.get()}
        </textarea>
        <button
            on:click=move |ev| submit(ev, data)>
            Salvar
        </button>
    }
}
