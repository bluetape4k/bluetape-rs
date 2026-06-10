# bluetape4k-core Collections Gap Analysis

## Source Scope

Reviewed Kotlin sources in `bluetape4k-projects/bluetape4k/core/src/main/kotlin/io/bluetape4k/collections`:

- `CollectionSupport.kt`
- `IterableSupport.kt`
- `IteratorSupport.kt`
- `ListSupport.kt`
- `MapEntrySupport.kt`
- `PaginatedList.kt`
- `SequenceSupport.kt`

Eclipse Collections adapters, primitive collection adapters, Java stream bridges,
and the lazy `Permutation` collection type were treated as out of scope for
`bluetape-rs-collections` 0.2.0 because they are JVM/library-specific or large
collection types rather than focused Rust backend helpers.

## Rust Standard Library Comparison

| bluetape4k helper family | Rust status | Decision |
| --- | --- | --- |
| `prepend`, `append`, `swap`, range-to-list, iterator-to-list | Covered by `Vec`, `VecDeque`, slice, range, and `Iterator::collect` APIs | Do not wrap |
| `safeSubList` | Slices have range indexing plus checked access patterns; owned clamped copy is policy-specific | Defer |
| `zipWithIndex`, `exists`, `size` | Covered by `Iterator::enumerate`, `Iterator::any`, `Iterator::count` / `ExactSizeIterator` | Do not wrap |
| primitive array conversion helpers | Rust uses typed iterators and `collect::<Vec<_>>()`; fallback casting is JVM-specific | Do not port |
| `sliding` / `windowed` for `Sequence` and `Iterable` | Slice exact windows exist; stable generic `Iterator` windows/chunks are not broadly available in Rust 1.85 | Implement owned iterator helpers |
| `chunkedBy` | No direct standard helper | Implement |
| `eachCount` | Can be written with `fold`, but no named frequency helper | Implement |
| `mapCatching`, `mapIfSuccess`, `forEachCatching` | Rust uses explicit `Result`; `collect::<Result<Vec<_>, _>>()` covers fail-fast collection | Implement only `partition_results` for collect-all successes/errors |
| map value transform | Standard APIs require manual loop or collect pattern | Implement `map_values` and `try_map_values` |
| `PaginatedList` | Domain model, not a collection helper | Defer to a pagination/domain crate decision |
| `safeSubList` follow-up | Standard slices offer checked access, but not signed bound clamping | Implement borrowed `clamped_subslice` |
| `padTo` follow-up | `Vec::resize` covers owned mutation, not borrowed zero-copy no-op padding | Implement `pad_to` returning `Cow<[T]>` |

## Implemented 0.2.0 Surface

- `iter::chunks`: lazy non-overlapping owned chunks for generic iterators.
- `iter::windows`: lazy exact overlapping owned windows for generic iterators.
- `iter::sliding_windows`: lazy overlapping owned windows with optional partial tails.
- `iter::chunked_by`: eager predicate-based chunk splitting where a matching item starts the next chunk.
- `iter::group_by`: group owned iterator items by a derived key.
- `iter::frequencies`: occurrence counts by item.
- `iter::partition_results`: unwrap `Result` items into `(Vec<T>, Vec<E>)`.
- `map::map_values`: transform `HashMap` values while preserving keys and hasher.
- `map::try_map_values`: fallible value transform with fail-fast `Result`.
- `slice::clamped_subslice`: borrowed clamped slice view for signed bounds.
- `slice::pad_to`: `Cow` padding helper that borrows when no padding is needed.

## Rationale

The implemented helpers intentionally do not duplicate slice APIs such as
`chunks` and `windows`; their value is for generic owned iterators where the
standard library cannot return borrowed overlapping slices. They also avoid
Kotlin/JVM exception-catching helpers because Rust callers should keep typed
`Result` contracts visible.

## Follow-up Issues

- #32: focused slice and list boundary helpers for clamped slicing and owned
  padding decisions. Implemented as `clamped_subslice` and `pad_to`.
- #33: Rust-native pagination value types, separated from collection helpers.
