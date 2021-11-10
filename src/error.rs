pub mod server_error {
    use trait_enum::*;
    #[allow(dead_code)]
    pub type Result<T> = std::result::Result<T, ServerError>;

    // Make it easy to convert strings to an error message
    pub trait ToServerError {
        fn to_chess_gui_error(&self) -> ServerError;
    }
    impl ToServerError for str {
        fn to_chess_gui_error(&self) -> ServerError {
            return ServerError::String(self.to_string());
        }
    }
    impl ToServerError for String {
        fn to_chess_gui_error(&self) -> ServerError {
            return ServerError::String(self.clone());
        }
    }

    use std::io::Error;
    trait_enum!{
        #[derive(Debug)]
        pub enum ServerError: ToString {
            String,
            Error
        }
    }

    impl std::fmt::Display for ServerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.to_string())
        }
    }

    impl std::convert::From<Error> for ServerError {
        fn from(err: Error) -> Self {
            return ServerError::Error(err);
        }
    }

    impl std::convert::From<String> for ServerError {
        fn from(err: String) -> Self {
            return ServerError::String(err);
        }
    }
    impl std::convert::From<&str> for ServerError {
        fn from(err: &str) -> Self {
            return ServerError::String(err.to_string());
        }
    }
}
