use std::fs::File;
use std::io::Read;

pub mod errors;
use errors::AppError;

/// Read a file and parse the first line as an integer.
pub fn read_first_number(path: &str) -> Result<i32, AppError> {
    let mut s = String::new();
    let mut f = File::open(path)?; // Io -> AppError via From
    f.read_to_string(&mut s)?;

    let first_line = s.lines().next().ok_or_else(|| AppError::NotFound(path.to_string()))?;
    let n: i32 = first_line.trim().parse()?; // ParseInt -> AppError via From
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn test_read_first_number_ok() {
        let path = "test_ok.txt";
        let _ = write(path, "42\nrest");
        let n = read_first_number(path).unwrap();
        assert_eq!(n, 42);
    }

    #[test]
    fn test_read_first_number_parse_error() {
        let path = "test_parse.txt";
        let _ = write(path, "notanumber");
        let res = read_first_number(path);
        assert!(res.is_err());
        match res.unwrap_err() {
            AppError::ParseInt(_) => (),
            e => panic!("expected ParseInt, got {:?}", e),
        }
    }

    #[test]
    fn test_read_first_number_not_found() {
        let path = "test_empty.txt";
        let _ = write(path, "");
        let res = read_first_number(path);
        assert!(res.is_err());
        match res.unwrap_err() {
            AppError::NotFound(_) => (),
            e => panic!("expected NotFound, got {:?}", e),
        }
    }
}
