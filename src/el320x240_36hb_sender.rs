use bytes::{BufMut, BytesMut};
use tokio::io::AsyncWriteExt;

pub(crate) async fn send_to_display(
    port: &mut tokio_serial::SerialStream,
    data: &[u8],
) -> std::io::Result<()> {
    const BLOCK_SIZE: usize = 64;
    const BLOCK_SIZE_PYLOAD: usize = BLOCK_SIZE - std::mem::size_of::<u32>();

    let chanks = data
        .chunks(BLOCK_SIZE_PYLOAD)
        .enumerate()
        .map(|(i, data)| {
            let offset = i * BLOCK_SIZE_PYLOAD;
            let mut buf = BytesMut::new();

            // offset in bytes
            buf.put_u32_le(offset as u32);
            buf.put_slice(data);

            buf
        });

    for buf in chanks {
        port.write_all(&buf).await?;
    }
    Ok(())
}
