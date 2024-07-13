tonic::include_proto!("users");
tonic::include_proto!("oauth");

/// User file descriptor
pub const USER_FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("users_descriptor");

/// Oauth file descriptor
pub const OAUTH_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("oauth_descriptor");
