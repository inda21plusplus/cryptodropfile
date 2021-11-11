pub mod server_error {
    use trait_enum::*;
    #[allow(dead_code)]
    pub type Result<T> = std::result::Result<T, ErrorEnum>;

    use std::io::Error;
    trait_enum!{
        #[derive(Debug)]
        pub enum ErrorEnum: ToString {
            String,
            Error
        }
    }

    impl std::fmt::Display for ErrorEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.to_string())
        }
    }

    impl std::convert::From<Error> for ErrorEnum {
        fn from(err: Error) -> Self {
            return ErrorEnum::Error(err);
        }
    }

    impl std::convert::From<String> for ErrorEnum {
        fn from(err: String) -> Self {
            return ErrorEnum::String(err);
        }
    }
    impl std::convert::From<&str> for ErrorEnum {
        fn from(err: &str) -> Self {
            return ErrorEnum::String(err.to_string());
        }
    }
}
