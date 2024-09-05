use std::fmt::Display;

pub trait ResultTrait<T> {
    fn map_to_string(self) -> Result<T, String>;
}

impl<T, E> ResultTrait<T> for Result<T, E>
where
    E: Display,
{
    fn map_to_string(self) -> Result<T, String> {
        self.map_err(|err| format!("{err}"))
    }
}
