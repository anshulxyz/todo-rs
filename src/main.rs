use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
    // initiate the cursive object
    let mut siv = cursive::default();

    // configure the cursive object
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(
        Dialog::text("text")
            .title("title")
            .button("Next", show_next),
    );

    // run the cursive object
    siv.run()
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Did you do the thang?!")
        .title("Question")
        .button("Yes", |s| ())
        .button("No", |s| ())
        .button("Quit", |s| s.quit())
    )
}