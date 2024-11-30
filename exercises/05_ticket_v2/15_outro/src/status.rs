// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = String;
    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        value.make_ascii_lowercase();
        if value == "todo".to_string() {
            return Ok(Status::ToDo);
        } else if value == "inprogress".to_string() {
            return Ok(Status::InProgress);
        } else if value == "done".to_string() {
            return Ok(Status::Done);
        }
        Err("Invalid input".to_string())
    }
}

impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value_str : String = String::from(value);
        value_str.make_ascii_lowercase();
        if value_str == "todo" {
            return Ok(Status::ToDo);
        } else if value_str == "inprogress" {
            return Ok(Status::InProgress);
        } else if value_str == "done" {
            return Ok(Status::Done);
        }
        Err("Invalid input".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
