fn main() {
    prost_build::compile_protos(
        &[
            "core/v1/core.proto",
            "foundation/v1/foundation.proto",
            "profile/v1/profile.proto",
        ],
        &["protocol/"],
    )
    .unwrap();
}
