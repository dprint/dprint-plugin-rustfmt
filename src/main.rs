use dprint_core::types::ErrBox;
use dprint_core::plugins::process::{
    get_parent_process_id_from_cli_args,
    handle_process_stdio_messages,
    start_parent_process_checker_thread,
};

mod plugin_handler;
mod format;
mod configuration;

use plugin_handler::*;
use format::*;

fn main() -> Result<(), ErrBox> {
    if let Some(parent_process_id) = get_parent_process_id_from_cli_args() {
        start_parent_process_checker_thread(env!("CARGO_PKG_NAME").to_string(), parent_process_id);
    }

    handle_process_stdio_messages(RustFmtPluginHandler::new())
}
