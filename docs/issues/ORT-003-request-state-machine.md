# ORT-003 Request State Machine

Implement request lifecycle semantics for waiting, prefill, decode, draining,
finished, aborted, and failed states.

## Requirements

- explicit transition table
- invalid transition errors
- idempotent abort
- exactly one terminal event per request

## Acceptance

- transition tests cover normal, abort, timeout, and failure paths
- event ordering is deterministic per request

