//! # Polkadot SDK Reference Docs.
//!
//! This is the entry point for all reference documents that enhance one's learning experience in
//! the Polkadot SDK.
//!
//! Note that this module also contains the [glossary](crate::reference_docs::glossary).
//!
//! ## What is a "reference document"?
//!
//! First, see [why we use rust-docs for everything](crate#why-rust-docs) and our documentation
//! [principles](crate#principles). We acknowledge that as much of the crucial information should be
//! embedded in the low level rust-docs. Then, high level scenarios should be covered in
//! [`crate::tutorial`]. Finally, we acknowledge that there is a category of information that is:
//!
//! 1. crucial to know.
//! 2. is too high level to be in the rust-doc of any one `type`, `trait` or `fn`.
//! 3. is too low level to be encompassed in a [`crate::tutorial`].
//!
//! We call this class of documents "reference documents". Our goal should be to minimize the number
//! of "reference" docs, as they incur maintenance burden.

/// Learn how Substrate and FRAME use traits and associated types to make modules generic in a
/// type-safe manner.
pub mod trait_based_programming;

/// Learn about the way Substrate and FRAME view their blockchains as state machines.
pub mod blockchain_state_machines;

/// The glossary.
pub mod glossary;

/// Learn about the WASM meta-protocol of all Substrate-based chains.
pub mod wasm_meta_protocol;

/// Learn about the differences between smart contracts and a FRAME-based runtime. They are both
/// "code stored onchain", but how do they differ?
pub mod runtime_vs_smart_contract;

/// Learn about how extrinsics are encoded to be transmitted to a node and stored in blocks.
pub mod extrinsic_encoding;

/// Learn about the signed extensions that form a part of extrinsics.
// TODO: @jsdw
pub mod signed_extensions;

/// Learn about *"Origin"* A topic in FRAME that enables complex account abstractions to be built.
// TODO: @shawntabrizi
pub mod frame_origin;

/// Learn about how to write safe and defensive code in your FRAME runtime.
// TODO: @CrackTheCode016
pub mod safe_defensive_programming;

/// Learn about composite enums in FRAME-based runtimes, such as "RuntimeEvent" and "RuntimeCall".
pub mod frame_composite_enums;

/// Learn about how to make a pallet/runtime that is fee-less and instead uses another mechanism to
/// control usage and sybil attacks.
pub mod fee_less_runtime;

/// Learn about metadata, the main means through which an upgradeable runtime communicates its
/// properties to the outside world.
// TODO: @jsdw
pub mod metadata;

/// Learn about how frame-system handles `account-ids`, nonces, consumers and providers.
pub mod frame_system_accounts;

/// Learn about the currency-related abstractions provided in FRAME.
pub mod frame_currency;

/// Learn about benchmarking and weight.
// TODO: @shawntabrizi @ggwpez
pub mod frame_benchmarking_weight;

/// Learn about chain specification file and the genesis state of the blockchain.
// TODO: @michalkucharczyk
pub mod chain_spec_genesis;

/// Learn about all the memory limitations of the WASM runtime when it comes to memory usage.
// TODO: @kianenigma
pub mod wasm_memory;

/// Learn about Substrate's CLI, and how it can be extended.
// TODO: @kianenigma
pub mod cli;

/// Learn about Substrate's consensus algorithms, and how you can switch between two.
// TODO: @JoshOrndorff @kianenigma
pub mod consensus_swapping;

/// Learn about all the advance ways to test your coordinate a rutnime upgrade and data migration.
// TODO: @liamaharon
pub mod frame_runtime_migration;

/// Learn about light nodes, how they function, and how Substrate-based chains come
/// light-node-first out of the box.
// TODO: @jsdw @josepot
pub mod light_nodes;
