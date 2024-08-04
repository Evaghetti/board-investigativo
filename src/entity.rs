use leptos::*;

pub struct EntityData {
    name: String,
    description: String,
    profile_url: String,
}

impl EntityData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: "".to_string(),
            // TODO: Upload de imagem pra não precisar pegar uma da internet
            profile_url:
                "https://i.pinimg.com/originals/f1/0f/f7/f10ff70a7155e5ab666bcdd1b45b726d.jpg"
                    .to_string(),
        }
    }
}

#[component]
pub fn EntityToken(data: EntityData) -> impl IntoView {
    // TODO
    view! {
        <section class="token">
            <img src=data.profile_url/>
            <h2>{data.name}</h2>
        </section>
    }
}
