pub use crate::error::server_error::Result;

#[allow(dead_code)]
pub fn write_file(data: &Vec<u8>, filename: &str) -> Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create(filename)?;
    file.write_all(data)?;
    return Ok(());
}

#[allow(dead_code)]
pub fn read_file(filename: &str) -> Result<Vec<u8>> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

#[allow(dead_code)]
pub fn get_paths(path: &str) -> Result<()> {
    use std::fs;
    let paths = fs::read_dir(path)?;

    for path in paths {
        println!("Name: {}", path?.path().display())
    }
    return Ok(());
}

#[allow(dead_code)]
pub fn get_files(path: &str) -> Result<()>  {
    use std::fs;
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap();
        if path.path().is_file() {
            println!("Name: {}", path.path().file_name().clone().unwrap().to_str().unwrap())
        }
    }
    return Ok(());
}

#[allow(dead_code)]
pub fn get_diretories(path: &str) -> Result<Vec<String>> {
    let mut return_list: Vec<String> = vec!();

    use std::fs;
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        if path.is_ok() {
            let path = path.unwrap();
            if path.path().is_dir() {
                let name = path.path().file_name().clone().unwrap().to_str().unwrap().to_string();
                return_list.push(name);
            }
        }
    }

    return Ok(return_list);
}