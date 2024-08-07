use leptos::*;
use uuid::Uuid;

pub const CARD_WIDTH_SIZE: i32 = 180;

#[derive(Clone, Copy, Debug)]
pub struct EntityData {
    id: Uuid,
    name: RwSignal<String>,
    // TODO: comentei só pra tirar os warnings, quando for fazer o editor de entidades recolocar
    // description: String,
    profile_url: RwSignal<String>,
}

impl EntityData {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            // TODO: Upload de imagem pra não precisar pegar uma da internet
            profile_url: create_rw_signal(
                "https://i.pinimg.com/originals/f1/0f/f7/f10ff70a7155e5ab666bcdd1b45b726d.jpg"
                    .to_string(),
            ),
            name: create_rw_signal("Fulano".to_string()),
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
