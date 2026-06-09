use std::{error::Error, fs, path::Path};

const PROTO_FILES: &[&str] = &[
    "opentelemetry/proto/common/v1/common.proto",
    "opentelemetry/proto/resource/v1/resource.proto",
    "opentelemetry/proto/trace/v1/trace.proto",
    "opentelemetry/proto/metrics/v1/metrics.proto",
    "opentelemetry/proto/logs/v1/logs.proto",
    "opentelemetry/proto/profiles/v1development/profiles.proto",
    "opentelemetry/proto/collector/trace/v1/trace_service.proto",
    "opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
    "opentelemetry/proto/collector/logs/v1/logs_service.proto",
    "opentelemetry/proto/collector/profiles/v1development/profiles_service.proto",
];

fn main() -> Result<(), Box<dyn Error>> {
    let proto_root = Path::new("opentelemetry/proto");
    if !proto_root.is_dir() {
        return Err("run this generator from the repository root".into());
    }

    let out_dir = Path::new("gen");
    fs::create_dir_all(out_dir)?;

    let mut config = prost_build::Config::new();
    config.out_dir(out_dir);
    config.compile_protos(PROTO_FILES, &["."])?;

    Ok(())
}
