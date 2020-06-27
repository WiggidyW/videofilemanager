use pyo3::types::PyDict;

pub struct GuessitPipe<'py> {
    py: pyo3::Python<'py>,
    modules: &'py PyDict,
}

pub type GuessitPipeError = pyo3::PyErr;

mod guessit_pipe {
    use super::GuessitPipe;
    use crate::token::Filename;
    use pyo3::PyErr;
    use pyo3::types::IntoPyDict;

    impl<'py> GuessitPipe<'py> {
        pub fn new(gil: &'py pyo3::GILGuard ) -> Result<Self, PyErr> {
            let py = gil.python();
            Ok(Self {
                modules: [
                    ("guessit", py.import("guessit")?),
                    ("json", py.import("json")?),
                    ("jsonutils", py.import("guessit.jsonutils")?),
                ].into_py_dict(py),
                py: py,
            })
        }
        fn guess<'a>(&'a self, filename: &Filename) -> Result<&'a str, PyErr> {
            self.py
                .eval(
                    &format!(
                        "json.dumps(guessit.guessit('{}'), cls=jsonutils.GuessitEncoder, ensure_ascii=False)",
                        **filename,
                    ),
                    None,
                    Some(self.modules),
                )?
                .extract()
        }
    }

    #[test]
    fn test_guess() {
        let gil = pyo3::Python::acquire_gil();
        let pipe = GuessitPipe::new(&gil).unwrap();
        let filename = Filename("Treme.1x03.Right.Place,.Wrong.Time.HDTV.XviD-NoTV.avi".to_string());
        let guess = pipe.guess(&filename);
        match guess {
            Ok(dict) => println!("{}", dict),
            Err(e) => {
                println!("{:?}", e);
                println!("{:?}", e.ptraceback);
                println!("{:?}", e.ptype);
            },
        }
    }
}