mod core;
mod query;
mod repl;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let is_interactive = args.len() == 2 && args[1] == "-i";

    if is_interactive {
        repl::start();
    } else {
        // TODO: listen on port
    }
}
