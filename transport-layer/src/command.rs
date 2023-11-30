
pub enum Command {
    // Place holder for serialization
    UNKNOWN,
    CD(String),
    MKDIR(String),
    LS,
    PWD,
    WHO,
}
impl Command{
    pub fn opt_code(&self) ->u8 {
        match self {
            Self::UNKNOWN => u8::MAX,
            Self::CD(..) => 1,
            Self::MKDIR(..) => 2,
            Self::PWD => 3,
            Self::LS => 4,
            Self::WHO =>5
        }
    }
    // Bytes sent on the wire
    // Format opt<u8>payloadlen<usize>payload
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        match self {
            Self::CD(target) =>{
                payload.push(self.opt_code());
                payload.push(target.len().try_into().unwrap());
                payload.extend(target.as_bytes().into_iter().clone());
            },
            Self::MKDIR(target) =>{
                payload.push(self.opt_code());
                payload.push(target.len().try_into().unwrap());
                payload.extend(target.as_bytes().into_iter().clone());
            },
            Self::LS => {
                payload.push(self.opt_code());
                payload.push(0u8);

            },
            Self::PWD => {
                payload.push(self.opt_code());
                payload.push(0u8);
            },
            Self::WHO => {
                payload.push(self.opt_code());
                payload.push(0u8);
            }
            Self::UNKNOWN =>{}
        };
        return payload

    }
    
}
impl From<(&str, &str)> for Command {
    fn from(value: (&str, &str)) -> Self {
        match value.0 {
            "cd" => Command::CD(value.1.to_string()),
            "mkdir" => Command::MKDIR(value.1.to_string()),
            "ls" => Command::LS,
            _ => Command::UNKNOWN
        } 
    }
}
impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value{
            "pwd" => Command::PWD,
            "whoami" => Command::WHO,
            "ls" => Command::LS,
            _ =>Command::UNKNOWN
        }
    }
    
}