use anyhow::Result;
use tokio::io::{stderr, stdin, stdout, AsyncReadExt, AsyncWriteExt, BufReader};

pub async fn read<R: AsyncReadExt + std::marker::Unpin>(
    reader: &mut BufReader<R>,
) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf).await?;
    Ok(buf)
}

pub async fn read_stdin() -> Result<String> {
    let mut reader = BufReader::new(stdin());
    read(&mut reader).await
}

pub async fn write<W: AsyncWriteExt + std::marker::Unpin>(writer: &mut W, b: &[u8]) -> Result<()> {
    writer.write_all(b).await?;
    Ok(())
}

pub async fn write_stdout(b: &[u8]) -> Result<()> {
    let mut writer = stdout();
    write(&mut writer, b).await
}

pub async fn write_stderr(b: &[u8]) -> Result<()> {
    let mut writer = stderr();
    write(&mut writer, b).await
}

pub fn is_pipe() -> bool {
    atty::isnt(atty::Stream::Stdin)
}

pub fn is_redirect() -> bool {
    atty::isnt(atty::Stream::Stdout)
}
