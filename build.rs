fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "core/v1/core.proto",
                "foundation/v1/foundation.proto",
                "profile/v1/profile.proto",
            ],
            &["protocol/"],
        )
        .unwrap();
}
