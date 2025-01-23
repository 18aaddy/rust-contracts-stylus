//! Common extensions to the ERC-721 standard.
pub mod burnable;
pub mod consecutive;
pub mod enumerable;
pub mod metadata;
pub mod uri_storage;
pub mod royalty;

pub use burnable::IErc721Burnable;
pub use enumerable::{Erc721Enumerable, IErc721Enumerable};
pub use metadata::{Erc721Metadata, IErc721Metadata};
pub use uri_storage::Erc721UriStorage;
