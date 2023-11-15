use wasm_bindgen::prelude::*;
use osmium_libs_lsp_handler::{Connection, Dispatcher};
use linter_server::create_linter;

#[wasm_bindgen]
pub fn create_extension(
    send_request: js_sys::Function,
    send_notification: js_sys::Function,
) -> Dispatcher {
    let creators = vec![create_linter];

    let connection = Connection::new(send_request, send_notification);
    let mut dispatcher = Dispatcher::new(connection);
    dispatcher.setup(creators);
    dispatcher
}
