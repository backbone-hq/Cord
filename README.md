# ![Cord](https://github.com/backbone-hq/cord/blob/master/media/cord.png?raw=true)

[![GitHub Build Status](https://img.shields.io/github/actions/workflow/status/backbone-hq/cord/ci.yml?branch=master)](https://github.com/backbone-hq/cord/actions/workflows/ci.yml)
[![GitHub License](https://img.shields.io/github/license/backbone-hq/cord)](https://github.com/backbone-hq/cord/blob/master/LICENSE.txt)
[![crates.io](https://img.shields.io/crates/v/cord)](https://crates.io/crates/cord)
![Made by Backbone](https://img.shields.io/badge/made_by-Backbone-blue)

Cord is a deterministic serialization format built in Rust, designed for security-sensitive applications where consistent and unambiguous binary representations are essential.

## 🏗️ Why Another Serialization Format?

Many serialization formats allow multiple binary representations of the same data (e.g., dictionaries with different key orders, or different integer encodings). This non-determinism creates problems when combining serialization with cryptographic operations like signing and hashing. **Cord guarantees that every unique semantic representation has exactly one unique binary representation.**

This deterministic approach is crucial for cryptographic use cases. When data needs to be signed or hashed, any variation in serialization — even between semantically equivalent representations — can produce different cryptographic results. This undermines the reliability of verification processes and introduces additional considerations during system design at best, or security vulnerabilities at worst.

Without deterministic serialization, systems face a burdensome choice: either store both the original serialized bytes alongside the deserialized data structures (doubling storage requirements and creating synchronization challenges), or risk the inability to verify previously signed data. This challenge becomes particularly acute in distributed systems where multiple parties need to independently verify signatures without access to the original serialized form.

Canonicalization solves this problem by ensuring that all participants, regardless of their implementation details, produce identical byte representations for identical data. This property allows cryptographic operations to be reliably repeatable across different implementations and environments.

The ability to have a single, deterministic binary representation for each unique data structure eliminates an entire class of potential inconsistencies and security issues. It means that verifiers can independently reconstruct the exact byte sequence that was signed, without needing to preserve the original serialization alongside the semantic content.

Cord's approach creates a foundation where cryptographic operations and data serialization work together seamlessly, rather than requiring complex workarounds to reconcile their different requirements.

## 💾 Installation

Cord is hosted on [crates.io](https://crates.io/crates/cord). You can add cord to your Rust project by running the `cargo` command below or by adding it to your `Cargo.toml`.

```bash
cargo add cord
```

## 📇 Basic Usage

```rust
use cord::{serialize, deserialize};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

// Instantiate and serialize a struct
let user = User {
    id: 42,
    name: "Alice".to_string(),
    active: true,
};
let bytes = serialize(&user).unwrap();

// Deserialize
let deserialized: User = deserialize(&bytes).unwrap();
assert_eq!(user, deserialized);
```

## 🧩 Supported Types

Cord intentionally limits its supported types to those that can be canonically represented:

| Type | Support | Notes |
|------|---------|-------|
| Boolean | ✅ | |
| Integers (i8, u8, i16, u16, etc.) | ✅ | Uses varint encoding |
| Strings | ✅ | UTF-8 with length prefix |
| Byte arrays | ✅ | With length prefix |
| Fixed-size sequences | ✅ | |
| Options | ✅ | |
| Struct/Tuple struct | ✅ | |
| Enums | ✅ | |
| Custom Set | ✅ | Canonically ordered |
| Custom DateTime | ✅ | UTC timestamp representation |
| Maps | ⏳ | Planned, but currently unsupported |
| Floating point | ❌ | Intentionally excluded due to NaN/representation issues |

## ☢️ Threat Model

Cord is designed to defend against scenarios where attackers exploit ambiguities in data representation to bypass security controls, particularly in cryptographic contexts. Examples of addressed threat vectors include:

1. **Canonicalization bypass**: Cryptographic systems often verify signatures against a normalized form while operating on raw input. Attackers exploit this gap by crafting inputs with trailing data, comment fields, or flexible encodings that bypass verification but execute differently. Classic examples include XML signature wrapping attacks and JWT header manipulation.
2. **Protocol confusion**: When data is parsed differently across system boundaries, attackers can craft inputs that pass one subsystem's verifications and authorize malicious actions in downstream systems, effectively amounting to a payload substitution attack.
3. **Inconsistency**: When third parties cannot independently reproduce the exact byte sequence of cryptographically authenticated data, verification becomes dependent on trusting the original signer's environment. In distributed verification systems like blockchains or certificate transparency logs, this can lead to novel failure modes such as consensus failures or validation errors.

Cord does **not** protect against:

- Side-channel attacks during serialization/deserialization
- Memory safety issues outside of Cord's implementation
- Malicious inputs exceeding reasonable size limits
- Implementation flaws in cryptographic primitives used with Cord outputs

## 🛰️ Advanced Example: Sets, Enums, and Custom Types

```rust
use cord::{serialize, deserialize, Set};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

// Custom type for document metadata
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Metadata {
    created_at: u64,
    author: String,
}

// Enum with different variants
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum AccessLevel {
    Public,
    Restricted(Vec<String>),
}

// Document type using a custom type, enum, and set
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Document {
    id: u32,
    metadata: Metadata,
    tags: Set<String>,
    access: AccessLevel,
}

fn main() {
    // Prepare a document to serialize
    let mut tags = HashSet::new();
    tags.insert("important".to_string());
    tags.insert("draft".to_string());
    
    let doc = Document {
        id: 42,
        metadata: Metadata {
            created_at: 1577836800,
            author: "Alice".to_string(),
        },
        tags: Set::from(tags),
        access: AccessLevel::Restricted(vec!["alice".to_string(), "bob".to_string()]),
    };

    // Serialize and deserialize
    let serialized = serialize(&doc).unwrap();
    let deserialized: Document = deserialize(&serialized).unwrap();

    // Tags are preserved but their internal representation is canonicalized
    assert_eq!(doc, deserialized);
}
```

## 🚧 Limitations and Trade-offs

Cord makes deliberate trade-offs to achieve its security properties:

1. **Backward compatibility**: The serialization format may subtly change between major versions
2. **Limited type support**: Complex types like floats are excluded to maintain determinism
3. **Performance cost**: Canonicalization introduces overhead compared to formats like FlatBuffers
4. **Additive schema evolution**: Fields cannot be removed once added without breaking compatibility
5. **No self-description**: Unlike formats like JSON, binary output is not human-readable and may have multiple interpretations under different schemas

## 📊 Current Status

Cord is a mature project that has seen production use in [Backbone](https://backbone.dev). Nevertheless, we urge users to:

- Thoroughly test before using in critical systems
- Be prepared for breaking changes in major versions
- Consider serialization format lock-in for long-term data storage

## ⏱️ Performance

While we don't yet have comprehensive benchmarks, initial testing shows Cord performs competitively with other Rust serialization formats. The varint encoding helps keep payload sizes small for common integer values.

However, be aware that the canonicalization process adds overhead compared to formats that don't guarantee canonical representations.

## 🗺️ Roadmap

Our current priorities are:

- Comprehensive fuzzing
- Support for additional common types
- Performance benchmarking and optimization
- Language bindings (Python and JavaScript first)
- Configurable limits for nested structures
- Formal verification of components

Anything else you'd like to see? [Suggest a feature](https://github.com/backbone-hq/cord/issues)!

---

Built with 🦴 by [Backbone](https://backbone.dev)
