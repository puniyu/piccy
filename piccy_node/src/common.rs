use image::Rgb;

pub(crate) fn parse_rgb(rgb_str: &str) -> napi::Result<Rgb<u8>> {
    let parts: Vec<&str> = rgb_str.split(',').collect();
    if parts.len() != 3 {
        return Err(napi::Error::from_reason("RGB string must have three components"));
    }

    let rgb: Result<Vec<u8>, _> = parts
        .iter()
        .map(|&part| {
            part.trim().parse::<u8>()
                .map_err(|_| napi::Error::from_reason("Invalid RGB component"))
        })
        .collect();

    let [r, g, b] = rgb?
        .try_into()
        .map_err(|_| napi::Error::from_reason("Failed to parse RGB components"))?;

    Ok(Rgb([r, g, b]))
}