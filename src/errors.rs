use std::fmt;

#[derive(Debug)]
pub struct ParallelismError {
    kind: ParallelismErrorKind,
}

impl ParallelismError {
    pub fn new(kind: ParallelismErrorKind) -> ParallelismError {
        ParallelismError { kind }
    }

    pub fn kind(&self) -> &ParallelismErrorKind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum ParallelismErrorKind {
    ZeroThreadsError,
    ThreadNumberError,
    ThreadAmountError,
}

impl fmt::Display for ParallelismError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            // "Thread number must be smaller than thread amount.\nThread number: {}\nThread amount: {}",
            ParallelismErrorKind::ZeroThreadsError => {
                write!(f, "there must be at least one thread")
            }
            // "Thread number must be smaller than thread amount.\nThread number: {}\nThread amount: {}",
            ParallelismErrorKind::ThreadNumberError => {
                write!(f, "thread number must be smaller than thread amount")
            }
            // "Total number of threads must be smaller than highest number.\nThreads amount: {}\nHighest number: {}.",
            ParallelismErrorKind::ThreadAmountError => write!(
                f,
                "total number of threads must be smaller than highest number"
            ),
        }
    }
}
