use leptos::*;

#[derive(Default, Clone)]
pub struct ConnectionLineData {
    pub id: u64,
    begin_position: RwSignal<(i32, i32)>,
    end_position: RwSignal<(i32, i32)>,
}

impl ConnectionLineData {
    pub fn new(begin_position: RwSignal<(i32, i32)>, end_position: RwSignal<(i32, i32)>) -> Self {
        Self {
            begin_position,
            end_position,
            ..Default::default()
        }
    }
}

#[component]
pub fn ConnectionLine(data: ConnectionLineData) -> impl IntoView {
    view! {
      <polygon
          points=move || format!("{},{} {},{}", data.begin_position.get().0, data.begin_position.get().1, data.end_position.get().0, data.end_position.get().1)
          style="fill:lime;stroke:purple;stroke-width:5;fill-rule:evenodd;"/>
    }
}
