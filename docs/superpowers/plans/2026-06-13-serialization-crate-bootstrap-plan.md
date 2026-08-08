# 직렬화 크레이트 부트스트랩 구현 계획

> **에이전트 작업자:** 필수 하위 스킬: 이 계획을 작업별로 구현하려면
> superpowers:subagent-driven-development(권장) 또는 superpowers:executing-plans를
> 사용한다. 단계 추적에는 checkbox(`- [ ]`) 문법을 사용한다.

**목표:** 검토된 부트스트랩 범위를 벗어나지 않고 이슈 #108을 위한 집중형
`bluetape-rs-serialization` workspace crate와 선택적 root facade feature를
추가한다.

**아키텍처:** 새 크레이트가 serialization vocabulary와 문서 경계를 소유하고,
root `bluetape-rs` crate는 opt-in facade feature를 통해서만 이를 노출한다.
이슈 #108은 첫 바이너리 어댑터 구현 전에 의도적으로 멈추며, 이후 `0.5.0`
이슈에서 이 크레이트 경계를 검토할 수 있게 된 뒤 계약/어댑터/테스트를
추가한다.

**기술 스택:** Rust 2024, Cargo workspace resolver 3, additive Cargo feature,
Rustdoc, README/README.ko 동등성.

---

## 파일 구조

- `crates/serialization/Cargo.toml` 생성: 패키지 메타데이터, 크레이트 이름 및
  비어 있는 기본 feature set.
- `crates/serialization/src/lib.rs` 생성: 자매 크레이트 스타일을 따르는 간결한
  crate-level Rustdoc. 긴 경계·마이그레이션·비목표 설명은 README/spec/plan
  문서에 둔다.
- `crates/serialization/README.md` 생성: 영어 사용자용 크레이트 경계, 사용 경로,
  불일치 의미론, 마이그레이션 메모 및 비목표.
- `crates/serialization/README.ko.md` 생성: 같은 섹션과 비목표 집합을 갖는
  한국어 로컬라이즈 크레이트 경계.
- `Cargo.toml` 수정: 기존 workspace member를 모두 보존하면서
  `crates/serialization`을 삽입하고 workspace dependency, 선택적 root
  dependency 및 `serialization` feature를 추가한다.
- `Cargo.lock` 수정: 로컬 workspace package 추가 후 갱신하고 이후 검증에는
  `--locked`를 사용한다.
- `src/lib.rs` 수정: `#[cfg(feature = "serialization")]` 뒤에 root facade
  re-export를 둔다.
- `README.md` 수정: 패키지 표 항목 이름을 바꾸고 direct-crate와 root-facade
  사용 지침을 추가한다.
- `README.ko.md` 수정: 로컬라이즈된 패키지 표와 지침을 동기화한다.
- `WIP.md` 검토: 병합된 `0.5.x` 분할 이후 이슈 #108에 추가 WIP 수정이 필요한지
  확인한다.

## 작업 1: 직렬화 크레이트 뼈대

**복잡도:** 낮음
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 생성: `crates/serialization/Cargo.toml`
- 생성: `crates/serialization/src/lib.rs`

- [x] **단계 1: 패키지 메타데이터 생성**

`crates/serialization/Cargo.toml`을 생성한다.

```toml
[package]
name = "bluetape-rs-serialization"
version = "0.4.0"
edition.workspace = true
rust-version.workspace = true
description = "Rust-native serialization contracts for bluetape-rs."
license.workspace = true
repository.workspace = true
readme = "README.md"
keywords = ["serialization", "serde", "binary", "cache"]
categories = ["encoding", "development-tools"]

[lib]
name = "bluetape_rs_serialization"
path = "src/lib.rs"

[features]
default = []

[dependencies]
```

버전 정책: 이슈 #108은 현재 개발 버전 계열에 반영한다. 릴리스 준비에서
크레이트 버전을 `0.5.0`으로 갱신할 때까지 패키지는 현재 workspace 버전을
유지하지만, 공개 README 예제는 게시하지 않은 `0.4.0` registry artifact를
호출자에게 가리켜서는 안 된다.

- [x] **단계 2: 간결한 크레이트 Rustdoc 생성**

`crates/serialization/src/lib.rs`를 생성한다.

```rust
//! Rust-native serialization boundary for bluetape-rs.
//!
//! This bootstrap crate reserves the focused `0.5.0` serialization package and
//! keeps the root facade opt-in. Serializer traits, payload envelopes, and
//! adapters are added in later reviewed issues.
//!
//! ```text
//! // Enable the root facade when a single dependency is more convenient:
//! // bluetape-rs = { version = "...", features = ["serialization"] }
//! ```

#[cfg(test)]
mod tests {
    #[test]
    fn crate_metadata_matches_serialization_boundary() {
        assert_eq!(env!("CARGO_PKG_NAME"), "bluetape-rs-serialization");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.4.0");
    }
}
```

이 형태는 `crates/core`, `crates/compression`, `crates/logging` 같은 기존 자매
크레이트 루트를 따른다. `lib.rs`에는 짧은 crate-level Rustdoc을 두고, 크레이트
루트에는 집중된 export/test를 두며, 자세한 사용법과 비목표는 README/spec/plan
문서에 둔다.

## 작업 2: Workspace 및 Facade 연결

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 수정: `Cargo.toml`
- 수정: `Cargo.lock`
- 수정: `src/lib.rs`

- [x] **단계 1: 배열을 교체하지 않고 workspace member 삽입**

`Cargo.toml`에서 기존 `crates/logging` member 다음에 아래 한 줄만 삽입하고,
benchmark 및 test crate를 포함한 기존 workspace member를 모두 보존한다.

```toml
    "crates/serialization",
```

- [x] **단계 2: workspace dependency 등록**

`[workspace.dependencies]`에 다음을 추가한다.

```toml
bluetape-rs-serialization = { path = "crates/serialization", version = "0.4.0" }
```

- [x] **단계 3: opt-in root feature 추가**

`[features]`에서 `default = ["core"]`는 그대로 두고 다음을 추가한다.

```toml
serialization = ["dep:bluetape-rs-serialization"]
```

- [x] **단계 4: 선택적 root dependency 추가**

`[dependencies]`에 다음을 추가한다.

```toml
bluetape-rs-serialization = { workspace = true, optional = true }
```

- [x] **단계 5: 게이트된 root facade 추가**

`src/lib.rs`에 다음을 추가한다.

```rust
#[cfg(feature = "serialization")]
pub use bluetape_rs_serialization as serialization;
```

- [x] **단계 6: locked 검증 전에 lock file을 한 번 갱신**

실행한다.

```bash
cargo check -p bluetape-rs-serialization --all-features
```

예상 결과:

- Cargo가 새 workspace package를 기록해야 하면 `Cargo.lock`이 갱신된다.
- 이후 검증 명령은 `--locked`를 사용한다.

- [x] **단계 7: 로컬에서 feature 연결 검증**

실행한다.

```bash
cargo metadata --no-deps --format-version 1 --locked
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --no-default-features --locked
cargo check -p bluetape-rs --features serialization --locked
```

예상 결과:

- 메타데이터에 기존 workspace member와 `crates/serialization`이 모두 포함된다.
- 메타데이터의 `bluetape-rs-serialization` 패키지에서 readme, license,
  repository 및 버전이 현재 개발 버전 정책과 일치한다.
- 일반 기본 `cargo check -p bluetape-rs --locked`는 기존 default feature set을
  계속 사용하며 serialization facade를 요구하지 않는다.
- `--no-default-features`는 facade를 요구하지 않는다.
- `--features serialization`은 선택적 facade dependency를 해석한다.

## 작업 3: 사용자용 문서 동등성

**복잡도:** 낮음
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 생성: `crates/serialization/README.md`
- 생성: `crates/serialization/README.ko.md`
- 수정: `README.md`
- 수정: `README.ko.md`
- 검토: `WIP.md`

- [x] **단계 1: 영어 크레이트 README 작성**

다음 섹션으로 `crates/serialization/README.md`를 생성한다.

````markdown
# bluetape-rs-serialization

Rust-native serialization boundary for `bluetape-rs`.

This crate is the bootstrap slice for the `0.5.0` cache-first binary
serialization milestone. It creates the package and documentation boundary first
so later issues can add traits, typed errors, binary payload envelopes, and
adapters behind reviewed feature flags.

## Usage

Direct crate dependency:

```toml
# Until 0.5.0 is published:
bluetape-rs-serialization = { git = "https://github.com/bluetape4k/bluetape-rs", package = "bluetape-rs-serialization" }

# After 0.5.0 is published:
bluetape-rs-serialization = "0.5"
```

```rust
use bluetape_rs_serialization as serialization;
```

Root facade usage:

```toml
# Until 0.5.0 is published:
bluetape-rs = { git = "https://github.com/bluetape4k/bluetape-rs", features = ["serialization"] }

# After 0.5.0 is published:
bluetape-rs = { version = "0.5", features = ["serialization"] }
```

```rust
use bluetape_rs::serialization;
```

The root facade is unavailable unless the `serialization` feature is enabled.
Default `bluetape-rs` builds remain unchanged.

The bootstrap crate exposes no serializer traits or adapters yet. Those arrive
in later reviewed `0.5.0` issues.

## Boundary

`0.5.0` starts with cache-first binary payload support. Payload metadata,
versioning, trust profiles, typed failures, and adapter contracts are added in
follow-up issues.

`Option<T>` represents absent values. Empty bytes are payload data and must not
be treated as a hidden null convention.

Future unsupported-version, wrong-format, and wrong-trust-profile cases are
typed decode failures. They must not decode as `None`, silently fall back, or
try alternate adapters. Cache eviction, namespace migration, and rebuild policy
belong to callers.

## Migration / Compatibility

Existing `bluetape-rs` users do not need code or Cargo changes for issue #108.
Callers only opt in by depending on `bluetape-rs-serialization` directly or by
enabling `features = ["serialization"]` on `bluetape-rs`.

## Issue #108 Bootstrap Non-goals

- Serializer traits or concrete adapters
- Runtime binary payload encoding
- Global serializer registry

## Not In The `0.5.0` Core/Binary Milestone

- JSON adapter
- Protobuf adapter
- Avro adapter
- Apache Fory adapter
- Testcontainers integration
- SQL or SQLx integration
- Resilience, retry, circuit-breaker, or fallback policies
- Unsafe deserialization
- Hidden global serializers
- Hidden default serializers
- Env-selected adapters
- Dynamic type loading
- Schema registry support
````

- [x] **단계 2: 완전한 동등성으로 한국어 크레이트 README 작성**

영어 README와 동일한 섹션 집합으로 `crates/serialization/README.ko.md`를
생성한다.

- `Usage`: 직접 의존성, `use bluetape_rs_serialization as serialization;`,
  루트 파사드 의존성, `use bluetape_rs::serialization;`, 기본 파사드를
  사용할 수 없다는 메모 및 경계만 포함하는 부트스트랩 메모.
- `Boundary`: `Option<T>`, 빈 바이트, unsupported-version, wrong-format,
  wrong-trust-profile의 타입 지정 실패, `None` fallback 금지, 대체 어댑터
  fallback 금지, 호출자가 소유하는 캐시 제거/네임스페이스 마이그레이션/재구축
  정책.
- `Migration / Compatibility`: 기존 `bluetape-rs` 사용자는 변경할 필요가 없으며
  직접 크레이트 또는 `features = ["serialization"]`으로 opt-in한다.
- `Issue #108 Bootstrap Non-goals`: trait/adapter 없음, 런타임 바이너리 페이로드
  인코딩 없음, 전역 레지스트리 없음.
- `Not In The 0.5.0 Core/Binary Milestone`: JSON, Protobuf, Avro, Fory,
  Testcontainers, SQL/SQLx, resilience/fallback 정책, 안전하지 않은 역직렬화,
  숨겨진 전역 serializer, 숨겨진 기본 serializer, 환경 선택 어댑터,
  dynamic type loading 및 schema registry 지원 제외.

- [x] **단계 3: root README 패키지 표 갱신**

`README.md`에서 serialization 패키지 이름을 바꾸고 benchmark 문구를 이후
benchmark 마일스톤으로 고정한다.

```markdown
| Serialization | `bluetape-rs-serialization` | Reserves the cache-first binary payload SerDe boundary first, then adds JSON, Protobuf, Avro, Fory, and the cross-repo benchmark track in `0.5.5` after adapters exist. |
```

패키지 표 근처에 짧은 메모를 추가한다.

```markdown
Use `bluetape-rs-serialization` directly for crate-level docs, or enable
`features = ["serialization"]` on `bluetape-rs` for the root facade.
Issue #108 is a crate/facade/docs bootstrap only; serializer traits, concrete
adapters, and runtime binary encoding arrive in later reviewed `0.5.0` issues.
```

- [x] **단계 4: 한국어 README 패키지 표 갱신**

`README.ko.md`에서 serialization 패키지 이름을 바꾸고 benchmark 문구를
`0.5.5`로 고정한다.

```markdown
| Serialization | `bluetape-rs-serialization` | Cache-first binary payload SerDe를 먼저 제공하고, JSON, Protobuf, Avro, Fory를 순차 확장한 뒤 adapter가 준비되면 `0.5.5`에서 cross-repo benchmark track을 진행합니다. |
```

패키지 표 근처에 이에 상응하는 한국어 메모를 추가한다.

```markdown
크레이트 수준 문서는 `bluetape-rs-serialization`을 직접 사용하고, 루트 파사드가
필요하면 `bluetape-rs`에서 `features = ["serialization"]`을 활성화하세요.
Issue #108은 크레이트/파사드/문서 부트스트랩만 수행합니다. Serializer 트레이트,
구체 어댑터, 런타임 바이너리 인코딩은 검토된 후속 `0.5.0` 이슈에서
추가합니다.
```

- [x] **단계 5: WIP 동등성 확인**

실행한다.

```bash
rg -n "bluetape-rs-serde|bluetape-rs-serialization|0\\.5\\.0|0\\.5\\.5" README.md README.ko.md WIP.md crates/serialization
```

예상 결과:

- 남은 `bluetape-rs-serde` 참조가 없다.
- `README.md`, `README.ko.md`, `WIP.md`, 크레이트 README 및 Rustdoc이 같은
  크레이트 이름, 루트 feature 이름 및 `0.5.0` 비목표 목록을 사용한다.
- 루트 README 파일에 같은 직접 크레이트/루트 파사드 지침을 넣거나, 자세한
  `Option<T>`, empty bytes, version/format/trust-profile mismatch 및 no-fallback
  의미론을 설명하는 crate README를 명시적으로 연결한다.
- `WIP.md`가 `0.5.x` 분할을 계속 문서화한다. 이슈 #108 부트스트랩 문구가
  없으면 milestone 분할이 이미 존재하므로 추가 WIP 수정이 필요하지 않았음을
  PR 본문에 기록한다.

## 작업 4: Feature 및 문서 검증

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 검증: `Cargo.toml`
- 검증: `Cargo.lock`
- 검증: `src/lib.rs`
- 검증: `crates/serialization/**`
- 검증: `README.md`
- 검증: `README.ko.md`
- 검증: `WIP.md`

- [x] **단계 1: 기본 dependency 제외 검증**

실행한다.

```bash
cargo tree -e features -p bluetape-rs --locked
cargo tree -e features -p bluetape-rs --no-default-features --locked
cargo tree -e features -p bluetape-rs --features serialization --locked
```

예상 결과:

- 일반 기본 tree에는 `bluetape-rs-serialization`이 포함되지 않는다.
- `--no-default-features`에도 `bluetape-rs-serialization`이 포함되지 않는다.
- `--features serialization`에는 `bluetape-rs-serialization`이 포함된다.
- 어떤 tree에도 이 부트스트랩의 JSON, Protobuf, Avro, Fory, Testcontainers,
  SQL 또는 resilience dependency가 포함되지 않는다.

- [x] **단계 2: facade 부재 및 존재 검증**

실행한다.

```bash
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --no-default-features --locked
cargo check -p bluetape-rs --features serialization --locked
```

예상 결과:

- default 및 no-default build는 `bluetape_rs::serialization`을 요구하지 않는다.
- root facade path는 `features = ["serialization"]`을 활성화한 경우에만
  사용할 수 있다. 아직 실행 가능한 doc/example으로 부재를 증명할 수 없다면,
  PR 생성 전에 Step 6-R에서 `src/lib.rs`를 검사하고
  `#[cfg(feature = "serialization")]` guard를 확인해야 한다.

- [x] **단계 3: 문서 및 예제 검증**

실행한다.

```bash
RUSTDOCFLAGS="-D warnings" cargo doc -p bluetape-rs-serialization --all-features --no-deps --locked
RUSTDOCFLAGS="-D warnings" cargo doc -p bluetape-rs --features serialization --no-deps --locked
```

예상 결과:

- Rustdoc이 경고를 거부한 상태로 빌드된다.
- direct crate usage, root facade usage, default-unavailable 메모,
  no-fallback/mismatch 동작 및 migration/compatibility 문구가 crate
  README/Rustdoc 쌍과 로컬라이즈 README에 나타난다.

- [x] **단계 4: 포맷 및 공백 검증**

실행한다.

```bash
cargo fmt --all --check
git diff --check
```

예상 결과:

- 포맷이 안정적이다.
- 공백 오류가 없다.

## 작업 5: Workspace 검증 및 검토 준비

**복잡도:** 중간
**필수 스킬:** `$bluetape-rs-patterns`

**파일:**
- 모든 변경 파일 검증.
- 생성: `docs/review/2026-06-13-issue-108-serialization-crate-review.md`
- 생성: `docs/lessons/2026-06-13-serialization-crate-bootstrap.md`

- [x] **단계 1: 이슈 인수 검증 실행**

실행한다.

```bash
cargo metadata --no-deps --format-version 1 --locked
cargo check -p bluetape-rs-serialization --all-features --locked
cargo test -p bluetape-rs-serialization --all-features --locked
cargo check -p bluetape-rs --locked
cargo check -p bluetape-rs --features serialization --locked
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
git diff --check
```

예상 결과:

- Step 5 검증 및 Step 6-R 코드 검토 전에 모든 명령이 통과한다.
- 메타데이터로 기존 workspace member가 보존되고
  `bluetape-rs-serialization`이 추가되었음을 확인한다.

- [x] **단계 2: 릴리스 준비 상태 경계 기록**

이슈 #108은 전체 `0.5.0` serializer 계열의 릴리스 준비 상태를 주장하지
않는다. 이후 릴리스 준비 상태를 주장하기 전에 다음과 같은 release-guide
dry-run 요구 사항을 실행하거나 기록한다.

```bash
cargo publish --workspace --dry-run --locked
```

이 PR에는 `cargo metadata`의 패키지 메타데이터 근거를 기록하고 게시
dry-run은 이후 `0.5.0` 릴리스 준비 상태 이슈의 범위임을 명시한다.

- [x] **단계 3: Step 2-R 보류 P2 항목을 후속 이슈로 전달**

이슈 #108에서는 바이너리 어댑터 bound를 구현하지 않는다.
`docs/review/2026-06-13-issue-108-serialization-crate-review.md`와 PR 본문에
이후 `0.5.0` 어댑터/릴리스 준비 상태 이슈가 다음 항목을 추적해야 한다고
기록한다.

- 인코딩 크기 제한(encoded-size limit)
- 압축 해제 크기 제한(decompressed-size limit)
- 압축 비율 제한(compression ratio limit)
- 컬렉션 경계(collection bound)
- 중첩/깊이 경계(nesting/depth bound)
- 손상된 바이트(corrupt bytes)
- 잘린 바이트(truncated bytes)
- 후행 바이트(trailing bytes)
- 빈 바이트(empty bytes)
- 알 수 없는 format id(unknown format id)
- content-type 불일치(content-type mismatch)
- 지원하지 않는 버전(unsupported version)
- 잘못된 대상 타입(wrong target type)
- trust-profile 불일치(trust-profile mismatch)
- 과도하게 큰 페이로드(oversized payload)
- 압축된 잘못된 페이로드(compressed-invalid payload)
- 어댑터 실패 사례(adapter failure cases)
- 실제 API가 노출된 뒤 실행하는 문서/예제 검사

이 항목들은 이후 어댑터 이슈 종료 또는 `0.5.0` 릴리스 준비 상태를
차단하지만, 이슈 #108 부트스트랩 완료를 차단하지는 않는다.

- [x] **단계 4: Step 6-R 코드 검토 근거 준비**

검토 범위:

- Cargo 워크스페이스 및 feature 연결.
- 루트 파사드 게이트.
- 크레이트 Rustdoc/README 경계.
- 루트 README 로케일 동등성.
- 기본 dependency 제외.
- 어댑터 코드 없음, 안전하지 않은 역직렬화 경로 없음, 전역/기본 레지스트리 없음.

예상 Step 6-R 게이트:

- 여섯 검토 관점과 현재 세션 통합.
- lesson, commit 또는 PR 생성 전에 `P0=0 P1=0`.
- 검토 근거를 `docs/review/2026-06-13-issue-108-serialization-crate-review.md`에
  저장.
- lesson을 `docs/lessons/2026-06-13-serialization-crate-bootstrap.md`에 저장.
- PR 본문 마지막 섹션은 `## DoD Status`이며 `P0=0 P1=0` 근거를 포함.

## 자체 검토

1. 사양 범위:
   - 이슈 #108 크레이트 부트스트랩, 워크스페이스 등록, 루트 파사드 게이트, 변경하지
     않은 기본값, 문서, README 동등성 및 명시적인 비목표가 위 작업에 모두
     대응한다.
   - 검토된 이슈 #108 범위에서 첫 바이너리 어댑터를 제외하므로 계획은 이를
     의도적으로 구현하지 않는다.
   - Step 2-R 보류 리소스 한도 및 문서/예제 검사는 이후
     어댑터/릴리스 준비 상태 작업에서 추적할 수 있다.
2. Placeholder 검사:
   - `TBD`, `TODO` 또는 모호한 구현 placeholder가 없다.
   - 보류한 binary-adapter runtime-bound 작업은 이 부트스트랩에 숨기지 않고
     이후 `0.5.0` 이슈에 명시적으로 배정했다.
3. 타입 일관성:
   - 패키지: `bluetape-rs-serialization`.
   - 라이브러리: `bluetape_rs_serialization`.
   - 루트 feature: `serialization`.
   - 루트 파사드 경로: `bluetape_rs::serialization`.
