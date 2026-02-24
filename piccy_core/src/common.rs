use bytes::Bytes;
use image::codecs::gif::Repeat;
use image::Frame;


pub(crate) fn encode_gif(frames: Vec<Frame>) -> crate::Result<Bytes> {
    let mut buffer = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut buffer);
        encoder.set_repeat(Repeat::Infinite)?;
        encoder.encode_frames(frames.into_iter())?;
    }
    Ok(buffer.into())
}
