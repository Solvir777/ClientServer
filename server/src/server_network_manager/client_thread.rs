use std::hash::{Hash, Hasher};
use tokio::net::tcp::OwnedWriteHalf;
use crate::server::UserId;

pub struct UserThread{
    id: UserId,
    write_half: OwnedWriteHalf,
}

impl Hash for UserThread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.id.to_be_bytes());
    }
}