pub mod log {
    tonic::include_proto!("grpc.log");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("log_descriptor");
}
