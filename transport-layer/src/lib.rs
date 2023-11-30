pub mod command;

#[cfg(test)]
mod tests {
    use crate::command::Command;

    #[test]
    fn test_cd_to_bytes() {
        let target_dir = "Documents";
        let command = Command::CD(target_dir.to_string());
        let out = command.to_bytes();
        let mut expected = vec![command.opt_code(), target_dir.len() as u8];
        expected.extend(target_dir.as_bytes());
        assert_eq!(out, expected)
    }
    #[test]
    fn test_ls_to_bytes() {
        let command = Command::LS;
        let out = command.to_bytes();
        let expected = vec![command.opt_code(), 0];
        assert_eq!(out, expected)
    }
}
