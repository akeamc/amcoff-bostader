use headers::{ContentType, Header};
use image::{DynamicImage, RgbaImage};
use pdfium_render::{
    error::PdfiumError, page::PdfPageRenderRotation, pdfium::Pdfium, prelude::PdfRenderConfig,
};
use reqwest::header::CONTENT_TYPE;

fn pdf_to_image(bytes: Vec<u8>) -> Result<DynamicImage, PdfiumError> {
    let pdfium = Pdfium::default();

    let document = pdfium.load_pdf_from_byte_vec(bytes, None)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    let page = document.pages().iter().next().unwrap();
    let bitmap = page.render_with_config(&render_config)?;

    Ok(bitmap.as_image())
}

#[derive(Debug, thiserror::Error)]
pub enum ToImageError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("image error")]
    Image(#[from] image::error::ImageError),
    #[error("pdfium error")]
    Pdfium(#[from] PdfiumError)
}

/// Attempt to convert an HTTP response to an [`RgbaImage`].
pub async fn to_image(res: reqwest::Response) -> Result<RgbaImage, ToImageError> {
    let content_type = ContentType::decode(&mut res.headers().get(CONTENT_TYPE).into_iter()).ok();

    // match content_type {
    //   Some(t) if t.
    // }

    // dbg!(content_type);

    let bytes = res.bytes().await?;

    tokio::task::spawn_blocking(move || {
        let img = image::load_from_memory(&bytes)
            .or_else(|_| pdf_to_image(bytes.to_vec()))?;

        Ok(img.to_rgba8())
    })
    .await
    .unwrap()
}
