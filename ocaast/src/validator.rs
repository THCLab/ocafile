pub trait Validator {
    fn validate(&self) -> Result<(Boolean), Error>;
}

pub struct OCAValidator {

}
