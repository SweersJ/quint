use quint_evaluator::itf::{Trace, TraceStatus};

fn trace_with_seed(seed: u64) -> Trace {
    Trace {
        states: vec![],
        status: TraceStatus::Ok,
        seed,
    }
}

#[test]
fn trace_metadata_includes_seed() {
    let json = serde_json::to_value(trace_with_seed(123).to_itf("model.qnt".to_string())).unwrap();

    assert_eq!(json["#meta"]["seed"], "123");
}

#[test]
fn trace_metadata_preserves_large_seed() {
    let json =
        serde_json::to_value(trace_with_seed(u64::MAX).to_itf("model.qnt".to_string())).unwrap();

    assert_eq!(json["#meta"]["seed"], u64::MAX.to_string());
}
