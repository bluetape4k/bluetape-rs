# 이슈 #75 압축 계약 검토

## 범위

- 이슈: #75, "Define compression error, config, and stream contracts"
- 마일스톤: 0.4.0
- 브랜치: `issue-75-compression-contracts`
- 기준선: `origin/develop`의 `431e892d0d476f66057b1ce5168a3bbb89e56de7`
- 검토 게이트: Step 6-R 로컬/네이티브 7-Tier 검토
- 검토 범위: `crates/compression`, 루트 파사드/docs, `README.md`, `README.ko.md`,
  `WIP.md`, `CHANGELOG.md`

## 7-Tier 결과

| 단계 | 에이전트 역할 | 결과 | 근거 |
|---|---|---:|---|
| 1 보안 | `security-reviewer` | PASS, P0=0 P1=0 | 64 MiB 기본 한도, decode 할당 전 lz4/snappy 선언 크기 거부, stream 한도 테스트 및 thread stress 범위를 확인했다. |
| 2 Ops/SRE 신뢰성 | `sre-reviewer` | PASS, P0=0 P1=0 | 재검토에서 결정론적으로 실패하는 `Read`/`Write` 테스트, 타입이 지정된 IO source, direct reader 한도 source 보존 및 기본 한도 assertion을 확인했다. |
| 3 구조/API | `architect-reviewer` | PASS, P0=0 P1=0 | 재검토에서 source 호환성을 위해 `decompress_with_config`가 기본 구현을 제공하고, 기존 형태의 사용자 지정 `Compressor` 구현자 테스트, no-default clippy 및 타입이 지정된 `UnsupportedOperation` fallback을 확인했다. |
| 4 Rust 코드 품질 | `code-reviewer` | PASS, P0=0 P1=0 | 타입이 지정된 오류/source, Rustdoc, feature cfg, production panic/todo 없음, stream constructor 및 boxed snappy writer enum 크기를 확인했다. |
| 5 테스트/타입 | `test-engineer` | PASS, P0=0 P1=0 | no-default clippy, direct `decompression_reader` 한도 테스트, 손상된 framed stream 실패 및 one-shot/stream/limit-failure stress 테스트를 확인했다. |
| 6 성능/안정성 | `performance-reviewer` | PASS, P0=0 P1=0 | lz4/snappy 사전 할당 보호, 제한된 stream copy 동작, 큰 enum boxing, exact-limit 테스트 및 stress 근거를 확인했다. |
| 7 문서/근거 | `library-user-reviewer` | PASS, P0=0 P1=0 | 재검토에서 registry Rustdoc가 lz4/snappy one-shot block/raw와 framed stream payload를 구분해 경고하고, trait 요약, README 동등성 및 rustdoc/doc test를 확인했다. |
| 최종 검증자 | `verifier` | PASS, P0=0 P1=0 | 이슈 요구 사항, 타입이 지정된 config/error/stream 계약, feature matrix, stress test, 문서 동등성 및 로컬 검증 근거를 확인했다. |

## 차단 요소 수렴

| 반복 | P0 | P1 | 해결 |
|---|---:|---:|---|
| 초기 Step 6-R | 0 | 6 | lz4/snappy 사전 할당 한도 검사, 64 MiB 기본 한도, 타입이 지정된 stream 오류 분류, stream constructor 및 Rustdoc/docs 동등성을 수정했다. |
| 영향 레인 재검토 | 0 | 3 | no-default clippy, IO failure 테스트, direct reader 한도의 타입이 지정된 source 및 공개 trait source 호환성을 수정했다. |
| 최종 영향 레인 재검토 | 0 | 0 | Tier 3 및 Tier 7의 잔여 차단 요소를 다시 검토해 문제가 없음을 확인했다. |

최종 게이트: PASS, `P0=0 P1=0`.

## 스트레스 테스트 근거

압축 통합 테스트 모음에 이제 제한된 멀티스레드 스트레스 테스트가 포함된다.

- `compiled_algorithms_round_trip_stays_stable_across_threads`
- `compiled_algorithms_stream_round_trip_stays_stable_across_threads`
- `compiled_algorithms_limit_failures_stay_stable_across_threads`

최근 집중 실행:

- `cargo test -p bluetape-rs-compression --all-features --locked`: PASS, 24 integration tests + 3 doctests

## 검증 근거

| 명령 | 결과 |
|---|---|
| `cargo fmt --all --check && git diff --check` | PASS |
| `cargo test -p bluetape-rs-compression --all-features --locked` | PASS, 24 integration tests + 3 doctests |
| `cargo test -p bluetape-rs-compression --no-default-features --locked` | PASS, 20 integration tests + 3 doctests |
| `for feature in gzip zlib deflate zstd lz4 snappy; do cargo test -p bluetape-rs-compression --no-default-features --features "$feature" --locked; done` | PASS |
| `cargo clippy -p bluetape-rs-compression --no-default-features --all-targets --locked -- -D warnings` | PASS |
| `cargo clippy -p bluetape-rs-compression --all-targets --all-features --locked -- -D warnings` | PASS |
| `rustup run 1.85.0 cargo check -p bluetape-rs-compression --all-targets --all-features --locked` | PASS |
| `cargo test --workspace --all-features --locked` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked` | PASS |
| `rustup run 1.85.0 cargo fmt --all --check && git diff --check && rustup run 1.85.0 cargo check --workspace --all-targets --all-features --locked && rustup run 1.85.0 cargo test -p bluetape-rs-compression --all-features --locked && rustup run 1.85.0 cargo clippy --workspace --all-targets --all-features --locked -- -D warnings && RUSTDOCFLAGS="-D warnings" rustup run 1.85.0 cargo doc -p bluetape-rs-compression --all-features --no-deps --locked` | PASS |

## 잔여 메모

- GitHub CI와 Step 7-R PR 후 검토는 이 로컬 Step 6-R 산출물에 포함되지
  않으며 PR 생성 후 실행해야 한다.
- 사용자 지정 `Compressor` 구현자의 기본 stream 메서드는 소스 호환성을 위한
  fallback이며 입력을 버퍼링한다. 운영 streaming 어댑터는 이를 override한다.
