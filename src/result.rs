#[allow(clippy::module_name_repetitions)]
pub type IResult<T> = Result<T, IError>;

#[derive(PartialEq, Eq, Debug)]
pub enum IError {
    ConversionError,
    BoundsError,
}
