use crate::DeckType;
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(
        long,
        env = "PHI_ADMIN_KEY",
        help = "The admin key unlocks special features in the UI when \
        passed as the `key` url parameter. \
        Defaults to a random value on startup when not specified."
    )]
    pub admin_key: Option<String>,
    #[structopt(
        long,
        env = "PHI_DISCONNECT_TIMEOUT_SECS",
        default_value = "3600",
        help = "Players that fail to send a heartbeat within this time will be \
        dropped from the game."
    )]
    pub disconnect_timeout_secs: u64,
    #[structopt(
        long,
        env = "PHI_DECK_TYPE",
        default_value = "fib",
        help = "Set the deck type: `fib` or `days`."
    )]
    pub deck_type: DeckType,
    #[structopt(long, env = "PHI_HTTP_ADDR", default_value = "0.0.0.0:7878")]
    pub http_addr: SocketAddr,
}
