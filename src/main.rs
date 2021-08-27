mod application;
mod routes;
mod tests;
use application::application;

fn main() {
    trillium_smol::run(application())
}
