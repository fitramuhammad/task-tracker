use task_tracker::{run, Config};
fn main() {
    let config = Config::build(std::env::args()).unwrap();

    let _ = run(config);
}
