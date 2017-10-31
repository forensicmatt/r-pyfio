use pyfio::PyFio;
use rwinreg::hive::Hive;
use cpython::{Python, PyObject, PyResult};
use std::io::{Read,Seek};
use std::cell::RefCell;

py_class!(class RegClass | py | {
    data inner: RefCell<Hive<Rs>>;

    def __new__(_self, fileio: PyObject) -> PyResult<RegClass> {
        let py_fio = PyFio::new(
            fileio
        );

        let hive = Hive::from_source(
            py_fio
        ).unwrap();

        RegClass::create_instance(py, hive)
    }
});
