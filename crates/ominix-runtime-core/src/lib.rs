//! Backend-neutral runtime contracts for OminiX inference.
//!
//! This crate intentionally avoids backend dependencies. CUDA, Ascend, and
//! SGLang adapters should implement these contracts without leaking device
//! details into the router or public API layers.

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RequestId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ModelId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct WorkerId(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestState {
    Waiting,
    Prefill,
    Decode,
    Draining,
    Finished,
    Aborted,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeErrorKind {
    AdmissionRejected,
    BackendUnavailable,
    CapabilityMismatch,
    KvCapacityExceeded,
    Timeout,
    Cancelled,
    Internal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerateRequest {
    pub request_id: RequestId,
    pub model: ModelId,
    pub input_ids: Vec<u32>,
    pub max_new_tokens: u32,
    pub stream: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendCapability {
    pub worker_id: WorkerId,
    pub backend_kind: BackendKind,
    pub model: ModelId,
    pub max_batch_tokens: u32,
    pub max_running_requests: u32,
    pub supports_prefill_decode_split: bool,
    pub supports_paged_kv: bool,
    pub supports_fp8_kv: bool,
    pub supports_graph_capture: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendKind {
    Sglang,
    OminiXCuda,
    OminiXAscend,
    Fake,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KvLease {
    pub request_id: RequestId,
    pub worker_id: WorkerId,
    pub blocks: Vec<KvBlockId>,
    pub tokens_reserved: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct KvBlockId(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchPlan {
    pub worker_id: WorkerId,
    pub phase: BatchPhase,
    pub requests: Vec<RequestId>,
    pub token_budget: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BatchPhase {
    Prefill,
    Decode,
    Mixed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeStep {
    pub batch: BatchPlan,
    pub kv_leases: Vec<KvLease>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerEvent {
    PrefillDone { request_id: RequestId },
    Token { request_id: RequestId, token_id: u32 },
    Usage { request_id: RequestId, prompt_tokens: u32, completion_tokens: u32 },
    Done { request_id: RequestId },
    Error { request_id: RequestId, error: RuntimeError },
}

pub trait BackendAdapter {
    fn capability(&self) -> BackendCapability;

    fn submit(&mut self, request: GenerateRequest) -> Result<RequestState, RuntimeError>;

    fn plan_decode_step(&mut self) -> Result<Option<DecodeStep>, RuntimeError>;

    fn poll_events(&mut self) -> Result<Vec<WorkerEvent>, RuntimeError>;

    fn abort(&mut self, request_id: &RequestId) -> Result<(), RuntimeError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeBackend {
        capability: BackendCapability,
        events: Vec<WorkerEvent>,
    }

    impl BackendAdapter for FakeBackend {
        fn capability(&self) -> BackendCapability {
            self.capability.clone()
        }

        fn submit(&mut self, request: GenerateRequest) -> Result<RequestState, RuntimeError> {
            self.events.push(WorkerEvent::PrefillDone {
                request_id: request.request_id.clone(),
            });
            self.events.push(WorkerEvent::Token {
                request_id: request.request_id,
                token_id: 42,
            });
            Ok(RequestState::Decode)
        }

        fn plan_decode_step(&mut self) -> Result<Option<DecodeStep>, RuntimeError> {
            Ok(None)
        }

        fn poll_events(&mut self) -> Result<Vec<WorkerEvent>, RuntimeError> {
            Ok(std::mem::take(&mut self.events))
        }

        fn abort(&mut self, _request_id: &RequestId) -> Result<(), RuntimeError> {
            Ok(())
        }
    }

    #[test]
    fn fake_backend_emits_prefill_and_token_events() {
        let mut backend = FakeBackend {
            capability: BackendCapability {
                worker_id: WorkerId("fake-0".to_string()),
                backend_kind: BackendKind::Fake,
                model: ModelId("deepseek-v4-flash".to_string()),
                max_batch_tokens: 4096,
                max_running_requests: 8,
                supports_prefill_decode_split: false,
                supports_paged_kv: true,
                supports_fp8_kv: true,
                supports_graph_capture: false,
            },
            events: Vec::new(),
        };

        let request_id = RequestId("req-1".to_string());
        let state = backend
            .submit(GenerateRequest {
                request_id: request_id.clone(),
                model: ModelId("deepseek-v4-flash".to_string()),
                input_ids: vec![1, 2, 3],
                max_new_tokens: 4,
                stream: true,
            })
            .unwrap();

        assert_eq!(state, RequestState::Decode);
        assert_eq!(
            backend.poll_events().unwrap(),
            vec![
                WorkerEvent::PrefillDone {
                    request_id: request_id.clone()
                },
                WorkerEvent::Token {
                    request_id,
                    token_id: 42
                }
            ]
        );
    }
}

