

fn main() {
    let root = "proto";
    let files = [
        "proto/metrics/v1/metrics.proto",
        "proto/collector/metrics/v1/metrics_service.proto",
        "proto/common/v1/common.proto",
        "proto/resource/v1/resource.proto"
    ];
    let mut cfg = prost_build::Config::new();
    cfg.out_dir("out");
    cfg.compile_protos(&files, &[root]).unwrap();


}
