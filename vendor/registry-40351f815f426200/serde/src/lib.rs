//! # Serde
//!
//! Serde is a framework for ***ser***ializing and ***de***serializing Rust data
//! structures efficiently and generically.
//!
//! The Serde ecosystem consists of data structures that know how to serialize
//! and deserialize themselves along with data formats that know how to
//! serialize and deserialize other things. Serde provides the layer by which
//! these two groups interact with each other, allowing any supported data
//! structure to be serialized and deserialized using any supported data format.
//!
//! See the Serde website [https://serde.rs/] for additional documentation and
//! usage examples.
//!
//! [https://serde.rs/]: https://serde.rs/
//!
//! ## Design
//!
//! Where many other languages rely on runtime reflection for serializing data,
//! Serde is instead built on Rust's powerful trait system. A data structure
//! that knows how to serialize and deserialize itself is one that implements
//! Serde's `Serialize` and `Deserialize` traits (or uses Serde's derive
//! attribute to automatically generate implementations at compile time). This
//! avoids any overhead of reflection or runtime type information. In fact in
//! many situations the interaction between data structure and data format can
//! be completely optimized away by the Rust compiler, leaving Serde
//! serialization to perform the same speed as a handwritten serializer for the
//! specific selection of data structure and data format.
//!
//! ## Data formats
//!
//! The following is a partial list of data formats that have been implemented
//! for Serde by the community.
//!
//! - [JSON], the ubiquitous JavaScript Object Notation used by many HTTP APIs.
//! - [Bincode], a compact binary format
//!   used for IPC within the Servo rendering engine.
//! - [CBOR], a Concise Binary Object Representation designed for small message
//!   size without the need for version negotiation.
//! - [YAML], a popular human-friendly configuration language that ain't markup
//!   language.
//! - [MessagePack], an efficient binary format that resembles a compact JSON.
//! - [TOML], a minimal configuration format used by [Cargo].
//! - [Pickle], a format common in the Python world.
//! - [RON], a Rusty Object Notation.
//! - [BSON], the data storage and network transfer format used by MongoDB.
//! - [Avro], a binary format used within Apache Hadoop, with support for schema
//!   definition.
//! - [JSON5], A superset of JSON including some productions from ES5.
//! - [URL], the x-www-form-urlencoded format.
//! - [Envy], a way to deserialize environment variables into Rust structs.
//!   *(deserialization only)*
//! - [Envy Store], a way to deserialize [AWS Parameter Store] parameters into
//!   Rust structs. *(deserialization only)*
//!
//! [JSON]: https://github.com/serde-rs/json
//! [Bincode]: https://github.com/TyOverby/bincode
//! [CBOR]: https://github.com/pyfisch/cbor
//! [YAML]: https://github.com/dtolnay/serde-yaml
//! [MessagePack]: https://github.com/3Hren/msgpack-rust
//! [TOML]: https://github.com/alexcrichton/toml-rs
//! [Pickle]: https://github.com/birkenfeld/serde-pickle
//! [RON]: https://github.com/ron-rs/ron
//! [BSON]: https://github.com/zonyitoo/bson-rs
//! [Avro]: https://github.com/flavray/avro-rs
//! [JSON5]: https://github.com/callum-oakley/json5-rs
//! [URL]: https://github.com/nox/serde_urlencoded
//! [Envy]: https://github.com/softprops/envy
//! [Envy Store]: https://github.com/softprops/envy-store
//! [Cargo]: http://doc.crates.io/manifest.html
//! [AWS Parameter Store]: https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-paramstore.html

////////////////////////////////////////////////////////////////////////////////

// Serde types in rustdoc of other crates get linked to here.
#![doc(html_root_url = "https://docs.rs/serde/1.0.92")]
// Support using Serde without the standard library!
#![cfg_attr(not(feature = "std"), no_std)]
// Unstable functionality only if the user asks for it. For tracking and
// discussion of these features please refer to this issue:
//
//    https://github.com/serde-rs/serde/issues/812
#![cfg_attr(feature = "unstable", feature(specialization, never_type))]
#![cfg_attr(feature = "alloc", feature(alloc))]
#![allow(unknown_lints, bare_trait_objects)]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
// Ignored clippy and clippy_pedantic lints
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        // not available in our oldest supported compiler
        const_static_lifetime,
        empty_enum,
        redundant_field_names,
        // integer and float ser/de requires these sorts of casts
        cast_possible_truncation,
        cast_possible_wrap,
        cast_precision_loss,
        cast_sign_loss,
        // things are often more readable this way
        cast_lossless,
        module_name_repetitions,
        single_match_else,
        type_complexity,
        use_self,
        zero_prefixed_literal,
        // not practical
        needless_pass_by_value,
        similar_names,
        // preference
        doc_markdown,
    )
)]
// Rustc lints.
#![deny(missing_docs, unused_imports)]

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "alloc")]
extern crate alloc;

/// A facade around all the types we need from the `std`, `core`, and `alloc`
/// crates. This avoids elaborate import wrangling having to happen in every
/// module.
mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::{cmp, iter, mem, num, slice, str};
    pub use self::core::{f32, f64};
    pub use self::core::{i16, i32, i64, i8, isize};
    pub use self::core::{u16, u32, u64, u8, usize};

    pub use self::core::cell::{Cell, RefCell};
    pub use self::core::clone::{self, Clone};
    pub use self::core::convert::{self, From, Into};
    pub use self::core::default::{self, Default};
    pub use self::core::fmt::{self, Debug, Display};
    pub use self::core::marker::{self, PhantomData};
    pub use self::core::ops::Range;
    pub use self::core::option::{self, Option};
    pub use self::core::result::{self, Result};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::borrow::{Cow, ToOwned};
    #[cfg(feature = "std")]
    pub use std::borrow::{Cow, ToOwned};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::{String, ToString};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;

    #[cfg(all(feature = "rc", feature = "alloc", not(feature = "std")))]
    pub use alloc::rc::{Rc, Weak as RcWeak};
    #[cfg(all(feature = "rc", feature = "std"))]
    pub use std::rc::{Rc, Weak as RcWeak};

    #[cfg(all(feature = "rc", feature = "alloc", not(feature = "std")))]
    pub use alloc::sync::{Arc, Weak as ArcWeak};
    #[cfg(all(feature = "rc", feature = "std"))]
    pub use std::sync::{Arc, Weak as ArcWeak};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
    #[cfg(feature = "std")]
    pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

    #[cfg(feature = "std")]
    pub use std::{error, net};

    #[cfg(feature = "std")]
    pub use std::collections::{HashMap, HashSet};
    #[cfg(feature = "std")]
    pub use std::ffi::{CStr, CString, OsStr, OsString};
    #[cfg(feature = "std")]
    pub use std::hash::{BuildHasher, Hash};
    #[cfg(feature = "std")]
    pub use std::io::Write;
    #[cfg(feature = "std")]
    pub use std::num::Wrapping;
    #[cfg(feature = "std")]
    pub use std::path::{Path, PathBuf};
    #[cfg(feature = "std")]
    pub use std::sync::{Mutex, RwLock};
    #[cfg(feature = "std")]
    pub use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(all(feature = "std", collections_bound))]
    pub use std::collections::Bound;

    #[cfg(core_reverse)]
    pub use self::core::cmp::Reverse;

    #[cfg(ops_bound)]
    pub use self::core::ops::Bound;

    #[cfg(range_inclusive)]
    pub use self::core::ops::RangeInclusive;

    #[cfg(any(core_duration, feature = "std"))]
    pub use self::core::time::Duration;
}

////////////////////////////////////////////////////////////////////////////////

#[macro_use]
mod macros;

#[macro_use]
mod integer128;

pub mod de;
pub mod ser;

#[doc(inline)]
pub use de::{Deserialize, Deserializer};
#[doc(inline)]
pub use ser::{Serialize, Serializer};

// Generated code uses these to support no_std. Not public API.
#[doc(hidden)]
pub mod export;

// Helpers used by generated code and doc tests. Not public API.
#[doc(hidden)]
pub mod private;

// Re-export #[derive(Serialize, Deserialize)].
//
// The reason re-exporting is not enabled by default is that disabling it would
// be annoying for crates that provide handwritten impls or data formats. They
// would need to disable default features and then explicitly re-enable std.
#[cfg(feature = "serde_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serde_derive")]
#[doc(hidden)]
pub use serde_derive::*;
