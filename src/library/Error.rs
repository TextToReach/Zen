use chumsky::error::Simple;
use thiserror::Error;
use miette::{Diagnostic, NamedSource, SourceSpan};

use crate::features::tokenizer::TokenData;

#[derive(Error, Debug, Diagnostic)]
#[error("Girinti Hatası")]
#[diagnostic(
    help("Blok kodlarda her blok için sadece +1 tab (\\t) kullanmaya dikkat edin.")
)]
pub struct GirintiHatası {
    #[source_code]
    pub src: NamedSource<String>,
	
    #[label("Hata buradan kaynaklandı.")]
    pub bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Değişken Bulunamadı")]
#[diagnostic(
    help("Bu değişkenin daha önceden tanımlanmış olduğundan emin olun.")
)]
pub struct DegiskenBulunamadı {
    #[source_code]
    pub src: NamedSource<String>,
	
    #[label("Hata buradan kaynaklandı.")]
    pub bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Fonksiyon Bulunamadı")]
#[diagnostic(
    help("Bu fonksiyonun daha önceden tanımlanmış olduğundan emin olun.")
)]
pub struct FonksiyonBulunamadı {
    #[source_code]
    pub src: NamedSource<String>,
	
    #[label("Hata buradan kaynaklandı.")]
    pub bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Tip Hatası")]
pub struct TipHatası {
    #[source_code]
    pub src: NamedSource<String>,
	
    #[label("Hata buradan kaynaklandı.")]
    pub bad_bit: SourceSpan,
    
    #[help("Beklenen tip: {expected:?}")]
    pub expected: Option<String>,
    
    #[help("Alınan tip: {got:?}")]
    pub got: Option<String>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Çok Fazla Argüman Hatası")]
pub struct CokFazlaArguman {
    #[source_code]
    pub src: NamedSource<String>,
	
    #[label("Hata buradan kaynaklandı.")]
    pub bad_bit: SourceSpan,
    
    #[help("Beklenen miktar: {expected:?}")]
    pub expected: Option<usize>,
    
    #[help("Alınan miktar: {got:?}")]
    pub got: Option<usize>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Çok Fazla Argüman Hatası")]
pub struct EksikArguman {
    #[source_code]
    pub src: NamedSource<String>,
	
    #[label("Hata buradan kaynaklandı.")]
    pub bad_bit: SourceSpan,
    
    #[help("Beklenen argüman: {expected:?}")]
    pub expected: Option<String>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Token Hatası")]
#[diagnostic(
    help("Beklenen token: {expected:#?}\nAlınan token: {got:?}")
)]
pub struct TokenHatası {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Hatalı token burada.")]
    pub bad_bit: SourceSpan,

    pub expected: Vec<String>,

    pub got: String,
}

impl TipHatası {
    pub fn expected(expected: String, got: String, src: NamedSource<String>, bad_bit: SourceSpan) -> Self {
        Self { src, bad_bit, expected: Some(expected), got: Some(got) }
    }
}