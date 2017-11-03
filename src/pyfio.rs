use cpython::{ Python, PyObject, PyBytes,
               PyTuple, ObjectProtocol};
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io;

#[derive(Debug)]
pub struct PyFio {
    io_object: Box<PyObject>
}

impl PyFio {
    pub fn new(fileio: PyObject)->PyFio{
        PyFio {
            io_object: Box::new(fileio)
        }
    }
}

// Warning! Gil must me aquired prior to calling!
impl Read for PyFio {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Get the assumed gil.
        let py = unsafe{
            Python::assume_gil_acquired()
        };

        // get buf length to read
        let buf_len = buf.len();

        // Read from fio
        let byte_buffer = match self.io_object.call_method(
            py, "read",
            (buf_len,),
            None
        ){
            Ok(buf) => buf,
            Err(error) => {
                let io_err = io::Error::new(
                    io::ErrorKind::Other,
                    format!("{:?}\n{}",error,backtrace!())
                );
                return Err(io_err);
            }
        };

        // We need to cast our 'str' to bytes
        let py_bytes = match byte_buffer.cast_into::<PyBytes>(py){
            Ok(bytes) => bytes,
            Err(error) => {
                let io_err = io::Error::new(
                    io::ErrorKind::Other,
                    format!("{:?}\n{}",error,backtrace!())
                );
                return Err(io_err);
            }
        };
        py_bytes.data(py).read(
            buf
        )?;

        Ok(buf.len())
    }
}
impl Seek for PyFio {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        // Get the assumed gil.
        let py = unsafe{
            Python::assume_gil_acquired()
        };

        match pos {
            SeekFrom::Current(n) => {
                match self.io_object.call_method(
                    py, "seek",
                    (n,1),
                    None
                ){
                    Ok(_) => {},
                    Err(error) => {
                        let io_err = io::Error::new(
                            io::ErrorKind::Other,
                            format!("{:?}\n{}",error,backtrace!())
                        );
                        return Err(io_err);
                    }
                }
            },
            SeekFrom::Start(n) => {
                match self.io_object.call_method(
                    py, "seek",
                    (n,0),
                    None
                ){
                    Ok(_) => {},
                    Err(error) => {
                        let io_err = io::Error::new(
                            io::ErrorKind::Other,
                            format!("{:?}\n{}",error,backtrace!())
                        );
                        return Err(io_err);
                    }
                }
            },
            other => {
                let io_err = io::Error::new(
                    io::ErrorKind::Other,
                    format!("{:?} is not currently supported.\n{}",
                        other, backtrace!())
                );
                return Err(io_err);
            }
        }

        let py_offset = match self.io_object.call_method(
            py, "tell",
            PyTuple::new(py,&[]),
            None
        ){
            Ok(buf) => buf,
            Err(error) => {
                let io_err = io::Error::new(
                    io::ErrorKind::Other,
                    format!("{:?}\n{}",error,backtrace!())
                );
                return Err(io_err);
            }
        };

        let offset = match py_offset.extract::<u64>(py){
            Ok(offset) => offset,
            Err(error) => {
                let io_err = io::Error::new(
                    io::ErrorKind::Other,
                    format!("{:?}\n{}",error,backtrace!())
                );
                return Err(io_err);
            }
        };

        Ok(offset)
    }
}
