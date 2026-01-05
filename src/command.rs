
pub trait OnceCommand {
    type Output;
    type Error: std::error::Error; // std::convert::Infallible

    fn execute(self) -> Result<Self::Output, Self::Error>;
}
