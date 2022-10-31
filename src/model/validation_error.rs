use std::fmt;

#[derive(Debug)]
pub enum ValidationErrorKind {
    SearchRangeStartAndEndEqualErrror,
    SearchRangeStartErrror,
    ThreadPanicError,
    ZeroThreadsError,
    ThreadNumberError,
    ThreadAmountError,
}

#[derive(Debug)]
pub struct ValidationError {
    kind: ValidationErrorKind,
}

impl ValidationError {
    pub fn new(kind: ValidationErrorKind) -> ValidationError {
        ValidationError { kind }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ValidationErrorKind::SearchRangeStartAndEndEqualErrror => {
                write!(f, "search range start and end can not be equal")
            }
            ValidationErrorKind::SearchRangeStartErrror => {
                write!(
                    f,
                    "search range start must be smaller than search range end"
                )
            }
            ValidationErrorKind::ThreadPanicError => {
                write!(f, "thread paniced")
            }
            ValidationErrorKind::ZeroThreadsError => {
                write!(f, "there must be at least one thread")
            }
            ValidationErrorKind::ThreadNumberError => {
                write!(f, "thread number must be smaller than thread amount")
            }
            ValidationErrorKind::ThreadAmountError => write!(
                f,
                "total number of threads must be smaller than highest number"
            ),
        }
    }
}
