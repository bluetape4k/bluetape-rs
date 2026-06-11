# bluetape-rs-async

[English](README.md) | [한국어](README.ko.md)

bluetape-rs를 위한 Tokio-first async task helper입니다.

![bluetape-rs-async crate overview](../../docs/images/readme-diagrams/bluetape-rs-async-crate.png)

이 crate는 `0.3.1` workspace release에 포함됩니다. Bounded task execution과
명시적 실패 동작을 위한 작은 helper를 제공합니다. Tokio primitive나
service-specific shutdown, timeout, deadline policy를 대체하지 않고, 흔한 task
lifecycle policy만 감쌉니다.

## 범위

- 명시적 maximum concurrency를 가진 bounded task scheduling
- 첫 오류 발생 시 sibling task를 abort하고 drain하는 first-error execution
- 성공과 operation error를 모두 기록하는 collect-all execution
- invalid bound와 Tokio task join failure를 위한 typed error
- `tokio::time` 기반 timeout 및 deadline wrapper
- owned Tokio watch channel 기반 cancellation 및 shutdown signal

이 crate는 core async task에서 blocking work를 실행하지 않습니다. Executor thread를
막을 수 있는 작업은 `tokio::task::spawn_blocking` 또는 service-specific worker
boundary를 사용하세요.

## 사용 예

```toml
[dependencies]
bluetape-rs-async = "0.3.1"
```

```rust
use bluetape_rs_async::try_map_bounded;

# async fn demo() -> Result<(), bluetape_rs_async::TaskGroupError<&'static str>> {
let values = try_map_bounded([1, 2, 3], 2, |value| async move {
    Ok::<_, &'static str>(value * 2)
})
.await?;

assert_eq!(values, vec![2, 4, 6]);
# Ok(())
# }
```

```rust
use std::time::Duration;

use bluetape_rs_async::with_timeout;

# async fn demo() -> Result<(), bluetape_rs_async::AsyncControlError> {
let value = with_timeout(Duration::from_millis(50), async { 42 }).await?;
assert_eq!(value, 42);
# Ok(())
# }
```
