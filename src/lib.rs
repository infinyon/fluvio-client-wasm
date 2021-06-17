mod admin;
mod connector;
mod consumer;
mod error;
mod fluvio;
mod offset;
mod partition;
mod producer;
mod record;
mod topic;

mod tests;

pub use crate::fluvio::Fluvio;
pub use admin::FluvioAdmin;
use connector::FluvioWebsocketConnector;
pub use consumer::{PartitionConsumer, PartitionConsumerStream};
pub use error::FluvioError;
pub use offset::Offset;
pub use producer::TopicProducer;
pub use record::Record;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
