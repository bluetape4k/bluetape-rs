# bluetape4k-core Collections 차이 분석

## 출처 범위

`bluetape4k-projects/bluetape4k/core/src/main/kotlin/io/bluetape4k/collections`의
Kotlin 소스를 검토했다.

- `CollectionSupport.kt`
- `IterableSupport.kt`
- `IteratorSupport.kt`
- `ListSupport.kt`
- `MapEntrySupport.kt`
- `PaginatedList.kt`
- `SequenceSupport.kt`

Eclipse Collections 어댑터, primitive collection 어댑터, Java stream bridge 및
lazy `Permutation` collection type은 JVM/라이브러리 전용이거나 집중형 Rust
백엔드 도우미가 아닌 대형 컬렉션 타입이므로 `bluetape-rs-collections` 0.2.0의
범위에서 제외했다.

## Rust 표준 라이브러리 비교

| bluetape4k 도우미 계열 | Rust 상태 | 결정 |
| --- | --- | --- |
| `prepend`, `append`, `swap`, range-to-list, iterator-to-list | `Vec`, `VecDeque`, slice, range 및 `Iterator::collect` API로 제공 | 래핑하지 않음 |
| `safeSubList` | Slice는 range indexing과 checked access 패턴을 제공하며, 소유 clamped copy는 정책에 따라 달라짐 | 보류 |
| `zipWithIndex`, `exists`, `size` | `Iterator::enumerate`, `Iterator::any`, `Iterator::count` / `ExactSizeIterator`로 제공 | 래핑하지 않음 |
| primitive array conversion helpers | Rust는 타입 지정 iterator와 `collect::<Vec<_>>()`를 사용하며 fallback casting은 JVM 전용 | 포팅하지 않음 |
| `Sequence` 및 `Iterable`의 `sliding` / `windowed` | Slice에는 정확한 window가 있지만 안정적인 generic `Iterator` windows/chunks는 Rust 1.85에서 널리 제공되지 않음 | 소유 iterator 도우미 구현 |
| `chunkedBy` | 직접 대응하는 표준 도우미 없음 | 구현 |
| `eachCount` | `fold`로 작성할 수 있지만 이름이 지정된 frequency 도우미는 없음 | 구현 |
| `mapCatching`, `mapIfSuccess`, `forEachCatching` | Rust는 명시적 `Result`를 사용하며 `collect::<Result<Vec<_>, _>>()`가 fail-fast 수집을 담당 | collect-all 성공/오류에는 `partition_results`만 구현 |
| map value transform | 표준 API에는 수동 loop 또는 collect 패턴이 필요 | `map_values` 및 `try_map_values` 구현 |
| `PaginatedList` | Rust에는 표준 page metadata 타입이 없지만 page value는 collection 결과에 유용한 domain-adjacent 타입 | 작은 Rust 네이티브 value type으로 구현 |
| `safeSubList` 후속 | 표준 slice는 checked access를 제공하지만 signed bound clamping은 제공하지 않음 | borrowed `clamped_subslice` 구현 |
| `padTo` 후속 | `Vec::resize`는 소유 변경을 지원하지만 borrowed zero-copy no-op padding은 지원하지 않음 | `Cow<[T]>`를 반환하는 `pad_to` 구현 |

## 구현한 0.2.0 표면

- `iter::chunks`: generic iterator용 lazy non-overlapping owned chunk.
- `iter::windows`: generic iterator용 lazy exact overlapping owned window.
- `iter::sliding_windows`: 선택적 partial tail을 지원하는 lazy overlapping owned window.
- `iter::chunked_by`: 일치하는 item이 다음 chunk를 시작하도록 predicate 기반으로 eager 분할.
- `iter::group_by`: 파생한 key로 owned iterator item을 그룹화.
- `iter::frequencies`: item별 발생 횟수.
- `iter::partition_results`: `Result` item을 `(Vec<T>, Vec<E>)`로 풀기.
- `map::map_values`: key와 hasher를 유지하면서 `HashMap` value 변환.
- `map::try_map_values`: fail-fast `Result`를 사용하는 fallible value 변환.
- `Page<T>`: zero-based page number, page size, total item count 및 계산된 total page count를 갖는 materialized page value type.
- `slice::clamped_subslice`: signed bound용 borrowed clamped slice view.
- `slice::pad_to`: padding이 필요 없을 때 borrow하는 `Cow` padding 도우미.

## 근거

구현한 도우미는 `chunks` 및 `windows` 같은 slice API를 의도적으로 중복하지
않는다. 표준 라이브러리가 borrowed overlapping slice를 반환할 수 없는
generic owned iterator에서 가치가 있기 때문이다. 또한 Rust 호출자가 타입이
지정된 `Result` 계약을 드러내야 하므로 Kotlin/JVM exception-catching 도우미를
피한다.

`Page<T>`는 Kotlin `PaginatedList` 데이터 계약을 유지하되 Rust value type으로
범위를 좁힌다. 음수가 아닌 page number와 count는 `u64`로 표현하고, 잘못된
page size는 `PageError`로 보고하며, 데이터베이스·SQL·cursor pagination 동작은
의미하지 않는다.

## 후속 이슈

- #32: clamped slicing 및 owned padding 결정을 위한 집중형 slice/list 경계
  도우미. `clamped_subslice` 및 `pad_to`로 구현했다.
- #33: Rust 네이티브 pagination value type. `bluetape-rs-collections`의
  `Page<T>`로 구현했다.
