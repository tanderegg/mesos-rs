extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/mesos.proto"],
                                //, "src/executor/src/executor.proto"],
                                &["src/"]).unwrap()
}
