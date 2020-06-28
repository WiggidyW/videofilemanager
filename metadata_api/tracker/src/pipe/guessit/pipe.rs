
use derive_more::{Display, Error, From};

pub struct GuessitPipe;

#[derive(Debug, Display, Error, From)]
pub enum GuessitPipeError {
    #[from(ignore)]
    PythonError(
        #[error(not(source))]
        String
    ),
    JsonError(serde_json::Error),
}

mod guessit_pipe {
    use super::{GuessitPipe, GuessitPipeError};
    use crate::token::Filename;
    use crate::pipe::Pipe;
    use crate::pipe::guessit::GuessitDict;
    use pyo3::types::IntoPyDict;
    use async_trait::async_trait;
    use std::sync::Arc;
    use std::convert::TryFrom;

    #[async_trait]
    impl Pipe<Filename, GuessitDict> for GuessitPipe {
        type Error = GuessitPipeError;
        type Stream = futures::stream::Iter<std::vec::IntoIter<Result<GuessitDict, Self::Error>>>;
        async fn get(self: &Arc<Self>, token: Filename) -> Result<Self::Stream, Self::Error> {
            Ok(futures::stream::iter(vec![Ok(
                self.guess(token)?
            )]))
        }
    }

    impl GuessitPipe {
        pub fn new() -> Self {
            Self
        }
        fn guess(&self, filename: Filename) -> Result<GuessitDict, GuessitPipeError> {
            let gil = pyo3::Python::acquire_gil();
            let py = gil.python();
            let modules = [
                    ("guessit", py.import("guessit")?),
                    ("json", py.import("json")?),
                    ("jsonutils", py.import("guessit.jsonutils")?),
                ]
                .into_py_dict(py);
            let code = format!(
r#"json.dumps(guessit.guessit('{}'), cls=jsonutils.GuessitEncoder, ensure_ascii=False)"#,
                *filename,
            );
            Ok(GuessitDict::try_from(
                py.eval(&code, None, Some(modules))?
                    .extract::<&str>()?
                )?
            )
        }
    }

    impl From<pyo3::PyErr> for GuessitPipeError {
        fn from(value: pyo3::PyErr) -> Self {
            Self::PythonError(format!("{:?}", value))
        }
    }

    #[test]
    fn test_guess() {
        let pipe = GuessitPipe::new();
        let filename = Filename("Treme.1x03.Right.Place,.Wrong.Time.HDTV.XviD-NoTV.avi".to_string());
        let guess = pipe.guess(filename);
        match guess {
            Ok(dict) => println!("{}", dict),
            Err(e) => println!("{}", e),
        }
    }
}