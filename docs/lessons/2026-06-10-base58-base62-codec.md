# Base58 And Base62 Codec Lessons

Date: 2026-06-10
Issue: #63

## What Happened

Base58 and Base62 were added as byte-oriented codec primitives instead of
mechanically porting the Kotlin/JVM object and extension API shape. Base58 uses
the Bitcoin alphabet and Base62 uses the bluetape numeric-first alphabet.

## What Surprised Us

The bluetape4k Kotlin line has two Base62 concepts: integer/UUID-oriented
`Base62` and KSUID-local byte-oriented `BytesBase62`. The Rust issue asked for
byte slices in and out, so the codec crate should expose byte-oriented
primitives now and leave integer/UUID rendering to a later ID-focused boundary.

The common Base58 sample `2NEpo7TZRRrLZSi2U` is for `Hello World!`, not
`Hello, World!`. Keep known-vector text exact when documenting examples.

## Next Time

- Name the alphabet and leading-zero policy in public docs for every base-family codec.
- Keep UUID and integer rendering out of `bluetape-rs-codec` until an ID-focused crate or module owns that API shape.
- Add thread stress tests for stateless codec paths when shared alphabets or conversion helpers are reused across codecs.
