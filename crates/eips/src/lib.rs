#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod eip1559;
pub use eip1559::calc_next_block_base_fee;

pub mod eip1898;
pub use eip1898::{
    BlockHashOrNumber, BlockId, BlockNumHash, BlockNumberOrTag, ForkBlock, RpcBlockHash,
};

pub mod eip2718;

pub mod eip2930;

pub mod eip2935;

pub mod eip4788;

pub mod eip4895;

pub mod eip4844;
pub use eip4844::{calc_blob_gasprice, calc_excess_blob_gas};

pub mod eip6110;
pub mod merge;

pub mod eip7002;

pub mod eip7685;

pub mod eip7702;
