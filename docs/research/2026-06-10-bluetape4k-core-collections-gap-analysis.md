# bluetape4k-core 컬렉션 격차 분석

## 소스 범위

다음 `bluetape4k-projects/bluetape4k/core/src/main/kotlin/io/bluetape4k/collections`
Kotlin source를 검토했습니다.

- `CollectionSupport.kt`
- `IterableSupport.kt`
- `IteratorSupport.kt`
- `ListSupport.kt`
- `MapEntrySupport.kt`
- `PaginatedList.kt`
- `SequenceSupport.kt`

Eclipse Collections adapter, primitive collection adapter, Java stream bridge,
lazy `Permutation` collection type는 JVM/library 전용이거나 큰 collection
type이며 집중형 Rust backend helper가 아니므로 `bluetape-rs-collections`
0.2.0 범위에서 제외했습니다.

## Rust 표준 라이브러리 비교

| bluetape4k helper family | Rust 상태 | 결정 |
| --- | --- | --- |
| `prepend`, `append`, `swap`, range-to-list, iterator-to-list | `Vec`, `VecDeque`, slice, range, `Iterator::collect` API로 제공 | wrap하지 않음 |
| `safeSubList` | slice에 range indexing과 checked access pattern이 있으며 owned clamped copy는 policy에 따라 다름 | 연기 |
| `zipWithIndex`, `exists`, `size` | `Iterator::enumerate`, `Iterator::any`, `Iterator::count` / `ExactSizeIterator`로 제공 | wrap하지 않음 |
| primitive array conversion helper | Rust는 typed iterator와 `collect::<Vec<_>>()`를 사용하며 fallback casting은 JVM 전용 | port하지 않음 |
| `sliding` / `windowed` for `Sequence` and `Iterable` | slice의 exact window는 존재하지만 stable generic `Iterator` window/chunk는 Rust 1.85에서 광범위하게 제공되지 않음 | owned iterator helper 구현 |
| `chunkedBy` | 직접 대응하는 standard helper 없음 | 구현 |
| `eachCount` | `fold`로 작성할 수 있지만 이름 있는 frequency helper는 없음 | 구현 |
| `mapCatching`, `mapIfSuccess`, `forEachCatching` | Rust는 명시적 `Result`를 사용하며 `collect::<Result<Vec<_>, _>>()`가 fail-fast collection을 처리 | collect-all success/error에는 `partition_results`만 구현 |
| map value transform | standard API에는 수동 loop 또는 collect pattern이 필요 | `map_values` 및 `try_map_values` 구현 |
| `PaginatedList` | Rust standard에는 page metadata type이 없지만 page value는 collection result에 유용한 domain-adjacent type | 작은 Rust 네이티브 value type으로 구현 |
| `safeSubList` 후속 | standard slice는 checked access를 제공하지만 signed bound clamping은 제공하지 않음 | borrowed `clamped_subslice` 구현 |
| `padTo` 후속 | `Vec::resize`는 owned mutation을 처리하지만 borrowed zero-copy no-op padding은 처리하지 않음 | `Cow<[T]>`를 반환하는 `pad_to` 구현 |

## 구현된 0.2.0 Surface

- `iter::chunks`: generic iterator용 lazy non-overlapping owned chunk.
- `iter::windows`: generic iterator용 lazy exact overlapping owned window.
- `iter::sliding_windows`: optional partial tail을 지원하는 lazy overlapping
  owned window.
- `iter::chunked_by`: matching item이 다음 chunk를 시작하는 eager
  predicate-based chunk splitting.
- `iter::group_by`: derived key로 owned iterator item을 group화.
- `iter::frequencies`: item별 occurrence count.
- `iter::partition_results`: `Result` item을 `(Vec<T>, Vec<E>)`로
  unwrap.
- `map::map_values`: key와 hasher를 유지하면서 `HashMap` value를 변환.
- `map::try_map_values`: fail-fast `Result`를 사용하는 fallible value
  transform.
- `Page<T>`: zero-based page number, page size, total item count, computed
  total page count를 포함하는 materialized page value type.
- `slice::clamped_subslice`: signed bound를 위한 borrowed clamped slice
  view.
- `slice::pad_to`: padding이 필요 없을 때 borrow하는 `Cow` padding helper.

## 근거

구현한 helper는 `chunks`, `windows` 같은 slice API를 의도적으로
중복하지 않습니다. standard library가 borrowed overlapping slice를 반환할 수
없는 generic owned iterator에서 가치가 있기 때문입니다. 또한 Rust caller가
typed `Result` contract를 드러내야 하므로 Kotlin/JVM exception-catching
helper를 피합니다.

`Page<T>`는 Kotlin `PaginatedList` data contract를 유지하면서 Rust value
type으로 범위를 좁힙니다. 음이 아닌 page number와 count는 `u64`로
표현하고 invalid page size는 `PageError`로 보고합니다. database, SQL,
cursor pagination behavior를 의미하지 않습니다.

## 후속 이슈

- #32: clamped slicing과 owned padding 결정을 위한 focused slice 및 list
  boundary helper. `clamped_subslice`와 `pad_to`로 구현했습니다.
- #33: Rust 네이티브 pagination value type.
  `bluetape-rs-collections`의 `Page<T>`로 구현했습니다.
