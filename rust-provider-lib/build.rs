fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        // FIXME: need to create pb directory first
        .out_dir("../pb")
        .compile(
            &[
                "../proto/core/core.proto",
                "../proto/core/error.proto",
                "../proto/core/provider.proto",
                "../proto/core/types.proto",
                "../proto/uno/uno.proto",
                "../proto/splendor/splendor.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}