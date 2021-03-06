pub mod arena;
pub mod ast;
pub mod editable_tree;
pub mod editor;

use crate::arena::Arena;
use crate::ast::json::JSONFormat;
use crate::ast::test_json::TestJSON;
use crate::editable_tree::DAG;
use crate::editor::Editor;

fn main() {
    // Initialise the logging and startup
    pretty_env_logger::init();
    log::info!("Starting up...");

    // Create an empty arena for Sapling to use
    log::trace!("Creating arena");
    let arena = Arena::new();
    // For the time being, start the editor with some pre-made JSON
    let root = TestJSON::Array(vec![
        TestJSON::True,
        TestJSON::False,
        TestJSON::Object(vec![("value".to_string(), TestJSON::True)]),
    ])
    .add_to_arena(&arena);

    let mut tree = DAG::new(&arena, root);
    let editor = Editor::new(&mut tree, JSONFormat::Pretty, editor::default_keymap());
    editor.run();
}
