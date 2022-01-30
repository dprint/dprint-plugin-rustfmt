use dprint_core::plugins::process::{
    get_parent_process_id_from_cli_args, handle_process_stdio_messages,
    start_parent_process_checker_thread,
};
use anyhow::Result;

mod configuration;
mod format;
mod plugin_handler;

use format::*;
use plugin_handler::*;

fn main() -> Result<()> {
    if let Some(parent_process_id) = get_parent_process_id_from_cli_args() {
        start_parent_process_checker_thread(parent_process_id);
    }

    handle_process_stdio_messages(RustFmtPluginHandler::new())
}
