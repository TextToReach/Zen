use thiserror::Error;
use miette::{Diagnostic, NamedSource, SourceSpan};

#[derive(Error, Debug, Diagnostic)]
#[error("Hay aksi!")]
#[diagnostic(
    help("Blok kodlarda her blok için sadece +1 tab (\\t) kullanmaya dikkat edin.")
)]
pub struct GirintiHatası {
    #[source_code]
    pub src: NamedSource<String>,
	
    #[label("Hata buradan kaynaklandı.")]
    pub bad_bit: SourceSpan,
}