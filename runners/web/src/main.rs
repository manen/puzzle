use yew::prelude::*;

#[function_component]
fn App() -> Html {
	let counter = use_state(|| 0);
	let onclick = {
		let counter = counter.clone();
		move |_| {
			let value = *counter + 1;
			counter.set(value);
		}
	};

	html! {
		<div>
			<button {onclick}>{ "+1" }</button>
			<p>{ *counter }</p>
		</div>
	}
}

fn main() {
	web_sys::window()
		.unwrap()
		.document()
		.unwrap()
		.set_title(puzzle_common::TITLE);

	yew::Renderer::<App>::new().render();
}
