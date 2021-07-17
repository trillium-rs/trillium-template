use crate::routes::{not_found, router};
use trillium::Handler;
use trillium_conn_id::{log_formatter::conn_id, ConnId};
use trillium_cookies::CookiesHandler;
use trillium_head::Head;
use trillium_logger::{dev_formatter, Logger, Target};
use trillium_method_override::MethodOverride;

pub fn application() -> impl Handler {
    (
        Logger::new()
            .with_target(Target::Stdout)
            .with_formatter((dev_formatter, " ", conn_id)),
        CookiesHandler::new(),
        Head::new(),
        MethodOverride::new(),
        ConnId::new(),
        router(),
        not_found,
    )
}
