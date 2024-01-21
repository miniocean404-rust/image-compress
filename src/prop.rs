#[derive(Copy, Clone, Default)]
pub struct JpegParameters {
    pub quality: u32,
    pub chroma_subsampling: ChromaSubsampling,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ChromaSubsampling {
    CS444,
    CS422,
    CS420,
    CS411,
    Auto,
}

impl Default for ChromaSubsampling {
    fn default() -> Self {
        ChromaSubsampling::Auto
    }
}

#[derive(Copy, Clone, Default)]
pub struct PngParameters {
    pub quality: u32,
    pub force_zopfli: bool,
}

#[derive(Copy, Clone, Default)]
pub struct GifParameters {
    pub quality: u32,
}

#[derive(Copy, Clone, Default)]
pub struct WebPParameters {
    pub quality: u32,
}

#[derive(Copy, Clone, Default)]
pub struct Props {
    pub jpeg: JpegParameters,
    pub png: PngParameters,
    pub gif: GifParameters,
    pub webp: WebPParameters,
    pub keep_metadata: bool,
    pub lossless: bool,
    pub width: u32,
    pub height: u32,
    pub output_size: u32,
}
