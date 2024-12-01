pub trait ProcessingErrorHandler
{
    fn callback(&mut self, line: u32, message: &str);
}
