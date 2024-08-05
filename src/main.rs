use leptos::*;

mod board;
mod entity;
mod line;

use board::BoardInvestigativo;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().expect("Não foi possível iniciar logs");

    mount_to_body(|| {
        view! {
            <BoardInvestigativo />
        }
    })
}
