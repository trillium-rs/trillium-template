use trillium::{conn_unwrap, Conn};
use trillium_router::{Router, RouterConnExt};

pub async fn hello_world(conn: Conn) -> Conn {
    conn.ok("hello world!")
}

pub async fn hello_name(conn: Conn) -> Conn {
    let name = conn_unwrap!(conn.param("name"), conn);
    let body = format!("hello, {}!", name);
    conn.ok(body)
}

pub async fn not_found(conn: Conn) -> Conn {
    let body = format!("Uh oh, I don't have a route for {}", conn.path());
    conn.with_body(body).with_status(404)
}

pub fn router() -> Router {
    Router::new()
        .get("/", hello_world)
        .get("/hello/:name", hello_name)
}
