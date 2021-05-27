mod utils;
mod admin;
mod record;
mod producer;
mod consumer;
mod offset;
mod error;
mod connector;
mod fluvio;

use connector::FluvioWebsocketConnector;
pub use error::FluvioError;
pub use offset::Offset;
pub use admin::FluvioAdmin;
pub use record::Record;
pub use producer::TopicProducer;
pub use consumer::{
    PartitionConsumerStream,
    PartitionConsumer,
};
use crate::fluvio::Fluvio;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

