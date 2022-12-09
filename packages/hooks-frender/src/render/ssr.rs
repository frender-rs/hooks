use futures_io::AsyncWrite;

#[non_exhaustive]
pub struct SsrWriter<'a, W: AsyncWrite + Unpin> {
    pub writer: &'a mut W,
    pub error: &'a mut Option<std::io::Error>,
}

pub struct SsrContext<'a, W: AsyncWrite + Unpin> {
    pub writer: &'a mut Option<SsrWriter<'a, W>>,
}
