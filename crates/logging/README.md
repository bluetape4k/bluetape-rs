# bluetape-rs-logging

Small `tracing` conventions and subscriber builders for bluetape-rs.

Library code must not install a process-global subscriber. Applications own the
decision to install the subscriber returned by this crate.
