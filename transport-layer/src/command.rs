#[derive(Debug, PartialEq, PartialOrd)]
pub enum Command {
    // Place holder for serialization
    UNKNOWN,
    CD(String),
    MKDIR(String),
    LS,
    PWD,
    WHO,
    RM(String),
    TOUCH(String),
    READ(String),
}
impl Command {
    pub fn opt_code(&self) -> u8 {
        match self {
            Self::UNKNOWN => u8::MAX,
            Self::CD(..) => 1,
            Self::MKDIR(..) => 2,
            Self::PWD => 3,
            Self::LS => 4,
            Self::WHO => 5,
            Self::RM(..) => 6,
            Self::TOUCH(..) => 7,
            Self::READ(..) => 8,
        }
    }
    // Bytes sent on the wire
    // Format opt<u8>payloadlen<usize>payload
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        match self {
            Self::LS => {
                payload.push(self.opt_code());
                payload.push(0u8);
            }
            Self::PWD | Self::WHO => {
                payload.push(self.opt_code());
                payload.push(0u8);
            }
            Self::TOUCH(target)
            | Self::RM(target)
            | Self::MKDIR(target)
            | Self::CD(target)
            | Self::READ(target) => {
                payload.push(self.opt_code());
                payload.push(target.len().try_into().unwrap());
                payload.extend(target.as_bytes().into_iter().clone());
            }

            Self::UNKNOWN => {}
        };
        return payload;
    }
}
impl From<(&str, &str)> for Command {
    fn from(value: (&str, &str)) -> Self {
        match value.0 {
            "cd" => Command::CD(value.1.to_string()),
            "mkdir" => Command::MKDIR(value.1.to_string()),
            "ls" => Command::LS,
            "rm" => Command::RM(value.1.to_string()),
            "read" => Command::READ(value.1.to_string()),
            _ => Command::UNKNOWN,
        }
    }
}
impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "pwd" => Command::PWD,
            "whoami" => Command::WHO,
            "ls" => Command::LS,
            _ => Command::UNKNOWN,
        }
    }
}
impl From<(u8, String)> for Command {
    fn from(value: (u8, String)) -> Self {
        match value.0 {
            1 => Command::CD(value.1),
            2 => Command::MKDIR(value.1),
            3 => Command::PWD,
            4 => Command::LS,
            5 => Command::WHO,
            6 => Command::RM(value.1),
            7 => Command::TOUCH(value.1),
            8 => Command::READ(value.1),
            _ => Command::UNKNOWN,
        }
    }
}
