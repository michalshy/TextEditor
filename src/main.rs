#![warn(clippy::all, clippy::pedantic, clippy::print_stdout, 
    clippy::arithmetic_side_effects, clippy::as_conversions, clippy::integer_division)]
use editor::Editor;

mod editor;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(first_arg) = args.get(1) {
        Editor::default().run(Some(first_arg));
    } else {
        Editor::default().run(None);
    }
}
