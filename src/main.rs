#[macro_use(slog_info)]
extern crate slog;
#[macro_use]
extern crate slog_scope;

mod cli;

use crater::{log, utils};
use failure::Fallible;
use std::panic;
use std::process;
use structopt::StructOpt;

fn main() {
    // Ignore errors loading `.env` file.
    let _ = dotenv::dotenv();

    let _guard = log::init();
    let success = match panic::catch_unwind(main_) {
        Ok(Ok(())) => true,
        Ok(Err(e)) => {
            utils::report_failure(&e);
            false
        }
        Err(e) => {
            utils::report_panic(&*e);
            false
        }
    };
    info!(
        "{}",
        if success {
            "command succeeded"
        } else {
            "command failed"
        }
    );
    log::finish();
    process::exit(if success { 0 } else { 1 });
}

fn main_() -> Fallible<()> {
    cli::Crater::from_args().run()
}
