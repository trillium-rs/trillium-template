mod application;
mod routes;
mod tests;
use application::application;

fn main() {
    pretty_env_logger::init();
    trillium_smol::run(application())
}
