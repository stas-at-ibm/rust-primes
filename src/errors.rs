use std::fmt;

#[derive(Debug)]
pub struct ValidationError {
    kind: ValidationErrorKind,
}

impl ValidationError {
    pub fn new(kind: ValidationErrorKind) -> ValidationError {
        ValidationError { kind }
    }

    pub fn kind(&self) -> &ValidationErrorKind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum ValidationErrorKind {
    SearchRangeStartAndEndEqualErrror,
    SearchRangeStartErrror,
    SendDataError,
    ThreadPanicError,
    ZeroThreadsError,
    ThreadNumberError,
    ThreadAmountError,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ValidationErrorKind::SendDataError => {
                write!(
                    f,
                    "could not send data from thread, receiver is disconnected "
                )
            }
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
            // "Thread number must be smaller than thread amount.\nThread number: {}\nThread amount: {}",
            ValidationErrorKind::ThreadNumberError => {
                write!(f, "thread number must be smaller than thread amount")
            }
            // "Total number of threads must be smaller than highest number.\nThreads amount: {}\nHighest number: {}.",
            ValidationErrorKind::ThreadAmountError => write!(
                f,
                "total number of threads must be smaller than highest number"
            ),
        }
    }
}
