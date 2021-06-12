use trillium::{conn_unwrap, Conn, Handler};
use trillium_logger::Logger;
use trillium_router::{Router, RouterConnExt};

async fn hello_world(conn: Conn) -> Conn {
    conn.ok("hello world!")
}

async fn hello_name(conn: Conn) -> Conn {
    let name = conn_unwrap!(conn, conn.param("name"));
    let body = format!("hello, {}!", name);
    conn.ok(body)
}

async fn not_found(conn: Conn) -> Conn {
    let body = format!("Uh oh, I don't have a route for {}", conn.path());
    conn.with_body(body).with_status(404)
}

fn application() -> impl Handler {
    (
        Logger::new(),
        Router::new()
            .get("/", hello_world)
            .get("/hello/:name", hello_name),
        not_found,
    )
}

fn main() {
    pretty_env_logger::init();
    {{runtime_crate_use}}::run(application())
}

#[cfg(test)]
mod tests {
    use super::application;
    use trillium_testing::prelude::*;

    #[test]
    fn says_hello_world() {
        assert_ok!(get("/").on(&application()), "hello world!")
    }

    #[test]
    fn says_hello_name() {
        assert_ok!(
            get("/hello/trillium").on(&application()),
            "hello, trillium!"
        );
        assert_ok!(get("/hello/rust").on(&application()), "hello, rust!");
    }

    #[test]
    fn other_routes_are_not_found() {
        assert_response!(
            get("/not/found").on(&application()),
            404,
            "Uh oh, I don't have a route for /not/found"
        )
    }
}
