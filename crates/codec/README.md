# bluetape-rs-codec

Codec and encoding helpers for bluetape-rs.

This crate starts the `0.3.0` codec milestone as a focused crate boundary. The
bootstrap change registers the crate in the workspace and root facade before
the first encoder APIs are added in follow-up issues.

## Scope

- strict hex encoding and decoding
- Base64 standard and URL-safe variants
- typed errors for caller-owned invalid encoded input
- small binary/text helpers when they make codec call sites clearer

## Out Of Scope

- compression helpers; those belong to `0.4.0`
- serde-oriented serialization interfaces; those belong to `0.5.0`
- encryption, signing, checksums, database bind encoding, and broad text
  normalization

## Usage

```toml
[dependencies]
bluetape-rs-codec = "0.3.0"
```

Or enable the optional root facade:

```toml
[dependencies]
bluetape-rs = { version = "0.1.1", features = ["codec"] }
```
