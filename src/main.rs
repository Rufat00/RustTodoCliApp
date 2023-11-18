use std::env;
use todo::helpers::file_exists;
use todo::helpers::reset_data;
use todo::return_error;
use todo::Config;
use todo::MyResultExt;

const DATA_FILE_PATH: &str = "data.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).handle_error();

    if !file_exists(DATA_FILE_PATH) {
        reset_data().handle_error();
    }

    match config.command.as_str() {
        "reset" => todo::commands::reset(config.flags).handle_error(),
        "list" | "l" => todo::commands::list().handle_error(),
        "new" | "n" => todo::commands::new(config.data, config.flags).handle_error(),
        "remove" | "r" => todo::commands::remove(config.data).handle_error(),
        "done" | "d" => todo::commands::set_done(config.data, true).handle_error(),
        "notdone" | "nd" => todo::commands::set_done(config.data, false).handle_error(),
        "update" | "u" => todo::commands::update(config.data, config.flags).handle_error(),
        _ => {
            return_error!(format!("`{}` command not exists", config.command))
        }
    }
}
