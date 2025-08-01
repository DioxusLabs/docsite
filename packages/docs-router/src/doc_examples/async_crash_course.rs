use dioxus::prelude::*;

fn solve_for_the_answer_to_life_and_everything() {}

fn unused() {
    // ANCHOR: async_block
    let future = async {
        println!("Ran");
    };
    // ANCHOR_END: async_block

    // ANCHOR: await
    let future = async {
        println!("Ran");
    };
    let other_future = async {
        future.await;
        println!("Ran Other");
    };
    spawn(other_future);
    // ANCHOR_END: await

    // ANCHOR: blocking
    spawn(async {
        // This will block the main thread and make the UI unresponsive.
        // Do not do this!
        solve_for_the_answer_to_life_and_everything();
        println!("Ran");
    });
    // ANCHOR_END: blocking

    // ANCHOR: thread
    std::thread::spawn(|| {
        // This will run on a separate thread and not block the main thread.
        solve_for_the_answer_to_life_and_everything();
        println!("Ran");
    });
    // ANCHOR_END: thread
}
