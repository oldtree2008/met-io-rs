use protobuf_codegen_pure::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=path/to/Cargo.lock");
    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            ..Default::default()
        })
        .out_dir("src/protos")
        .input("src/protos/examples.proto")
        .input("src/protos/nom.proto")
        .include("src/protos")
        .run()
        .expect("protoc");
}
