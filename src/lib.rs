use std::ffi::CString;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure Python IO works interactively
    // The communication with evcxr is done via a pipe, so by default it is buffered
    // Using this might be an alternative, but it is an unsafe "static mut"
    // pyo3::ffi::Py_UnbufferedStdioFlag = 1;
    std::env::set_var("PYTHONUNBUFFERED", "1");

    // Initialize Python
    pyo3::prepare_freethreaded_python();

    // Ensure the Python symbols will be available for other libraries
    // First get the Python dynamic library by querying the Python executable
    // Then dlopen it with RTLD_GLOBAL
    let libpython: pyo3::PyResult<String> = pyo3::Python::with_gil(|py| {
        let res = py.eval(
            r"import sysconfig; sysconfig.get_config_var('LDLIBRARY')",
            None,
            None,
        )?;
        res.extract()
    });
    // Bind the CString to a variable, such that the value is kept alive during the `as_ptr`
    let libpython = match libpython {
        Ok(file) => CString::new(file)?,
        Err(_) => CString::new("libpython3.so")?,
    };

    unsafe {
        libc::dlopen(libpython.as_ptr(), libc::RTLD_GLOBAL | libc::RTLD_NOW);
    }

    Ok(())
}
