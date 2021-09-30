fn main() {
    tonic_build::configure()
    .build_server(true)
    .compile(
        &["proto/opentelemetry/proto/collector/trace/v1/trace_service.proto"], &["proto/"]
    )
    .unwrap();
}