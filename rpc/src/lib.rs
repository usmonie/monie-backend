pub mod monie {
    pub mod media {
        tonic::include_proto!("media");
    }


    pub mod user {
        tonic::include_proto!("user");
    }

    pub mod auth {
        tonic::include_proto!("authentication");
    }
}

