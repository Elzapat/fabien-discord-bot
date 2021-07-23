// use serenity::framework::standard::CommandError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FabiError {
    // CommandError(CommandError),
    NotInAGuild,
    InvalidArgument,
    MissingMember,
}

// pub type FabiError = Box<dyn Error + Send + Sync>;
// pub type FabiResult<T = ()> = Result<T, Box<FabiError>>;

impl Error for FabiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
        // match self {
        //     FabiError::CommandError(e) => Some(&e),
        //     MissingData => Some("Some data was expected but was missing"),
        // }
    }
}

impl fmt::Display for FabiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl Into<FabiError> for Option::None {
//     fn into(self) -> FabiError {
//         Box::new(Err("test"))
//     }
// }

// impl Into<CommandError> for FabiError {
//     fn into(self) -> CommandError {
//         match self {
//             FabiError::CommandError(e) => e,
//         }
//     }
// }
//
// impl From<CommandError> for FabiError {
//     fn from(err: CommandError) -> FabiError {
//         FabiError::CommandError(err)
//     }
// }
