pub mod command;

#[cfg(test)]
mod tests {
    use crate::command::Command;

    #[test]
    fn test_cd_to_bytes() {
        let target_dir = "Documents";
        let user = 100u8;
        let command = Command::CD(target_dir.to_string());
        let out = command.to_bytes(user);
        let mut expected = vec![user, command.opt_code(), target_dir.len() as u8];
        expected.extend(target_dir.as_bytes());
        assert_eq!(out, expected)
    }
    #[test]
    fn test_ls_to_bytes() {
        let command = Command::LS;
        let user = 100u8;
        let out = command.to_bytes(user);
        let expected = vec![user, command.opt_code(), 0];
        assert_eq!(out, expected)
    }
    #[test]
    fn test_from_opt() {
        let opts = vec![
            (1u8, "Documents".to_string()),
            (2, "NewDir".to_string()),
            (3, "".to_string()),
        ];
        let expected = vec![
            Command::CD("Documents".to_string()),
            Command::MKDIR("NewDir".to_string()),
            Command::PWD,
        ];
        let out = opts
            .into_iter()
            .map(|o| Command::from(o))
            .collect::<Vec<Command>>();
        assert_eq!(expected, out)
    }
}
