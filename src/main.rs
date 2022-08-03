use cursive::views::{DummyView, LinearLayout, TextView};

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let tv1 = TextView::new("Hello cursive! Press <q> to quit.");
    let tv2 = TextView::new("You still know rust worse than I do!");
    siv.add_layer(
        LinearLayout::vertical()
            .child(tv1)
            .child(DummyView)
            .child(tv2),
    );

    siv.run();
}
