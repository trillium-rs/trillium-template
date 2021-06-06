use trillium::{Conn, Handler};
use trillium_logger::Logger;

async fn hello_world(conn: Conn) -> Conn {
    conn.ok("hello world!")
}

fn application() -> impl Handler {
    (Logger::new(), hello_world)
}

fn main() {
    pretty_env_logger::init();
    trillium_smol::run(application())
}

#[cfg(test)]
mod tests {
    use super::hello_world;
    use trillium_testing::prelude::*;

    #[test]
    fn says_hello_world() {
        assert_ok!(get("/").on(&hello_world), "hello world!")
    }
}
