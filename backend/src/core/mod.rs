pub mod api;
pub mod forms;
pub mod guards;
pub mod lifecycle;
pub mod response;
pub mod roles;
pub mod runtime;
pub mod server;
pub mod state;

use crate::constants::SECRET_KEY_LENGTH;
use base64::Engine;
use rand::Rng;

pub use state::State;

pub fn generate_secret_key() -> String {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..SECRET_KEY_LENGTH).map(|_| rng.gen()).collect();
    base64::engine::general_purpose::STANDARD.encode(key)
}
