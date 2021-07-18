mod application;
mod routes;
use application::application;

fn main() {
    pretty_env_logger::init();
    trillium_smol::run(application())
}

mod tests;
