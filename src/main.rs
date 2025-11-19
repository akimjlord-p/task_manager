mod utils;
mod cli;
use cli::CliApp;


fn main() {
    let mut app = CliApp::initiate();
    app.start_cycle();
}
