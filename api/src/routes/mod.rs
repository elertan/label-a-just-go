pub mod api;

#[get("/ping")]
pub fn ping() -> &'static str {
    "Pong!"
}

