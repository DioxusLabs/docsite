//! Easily integrate async Rust code into your components.

fn Tasks(cx: Scope) -> Element {
	let count = use_state(cx, || 0);

	use_coroutine(cx, |_| {
		to_owned![count];
		async move {
			loop {
				count += 1;
				tokio::time::sleep(Duration::from_millis(500)).await;
			}
		}
	});

	render!( pre { "Count: {count}" } )
}
