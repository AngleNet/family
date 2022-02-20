pub struct FamilyError {
    pub msg: String,
}

impl FamilyError {
    pub fn with(msg: String) -> FamilyError{
        FamilyError { msg }
    }
}

pub type Result<T> = std::result::Result<T, FamilyError>;
