pub fn get_exe_path() -> Result<String, std::io::Error> {
    let path_buf_result = std::env::current_exe();
    match path_buf_result {
        Ok(path_buf) => match path_buf.as_path().parent().unwrap().to_str() {
            Some(path_str) => Ok(String::from(path_str)),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "get file name from path_buf failed",
            )),
        },
        Err(err) => Err(err),
    }
}
