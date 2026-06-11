# bluetape-rs-logging

[English](README.md) | [한국어](README.ko.md)

bluetape-rs를 위한 작은 `tracing` convention과 subscriber builder입니다.

![bluetape-rs-logging crate overview](../../docs/images/readme-diagrams/bluetape-rs-logging-crate.png)

Library code는 process-global subscriber를 설치하지 않아야 합니다. 이 crate가
반환하는 subscriber를 설치할지 결정하는 책임은 application에 있습니다.

## 사용 예

```toml
[dependencies]
bluetape-rs-logging = "0.3.1"
```

```rust
use bluetape_rs_logging::CorrelationId;

let correlation_id = CorrelationId::new("request-1").expect("valid id");
assert_eq!(correlation_id.as_str(), "request-1");
```
