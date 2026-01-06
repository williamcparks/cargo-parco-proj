use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjError {
    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error("Key `{0}` In proj.env.base Is Over Writing It's Value In proj.env.dev")]
    BaseOverWriteDev(Box<str>),

    #[error("Key `{0}` In proj.env.base Is Over Writing It's Value In proj.env.prod")]
    BaseOverWriteProd(Box<str>),

    #[error("Key `{0}` Is In proj.env.dev But Not In proj.env.prod")]
    KeyInDevNotInProd(Box<str>),

    #[error("Key `{0}` Is In proj.env.prod But Not In proj.env.dev")]
    KeyInProdNotInDev(Box<str>),

    #[error("Key `{0}` Not Found In proj.env")]
    KeyNotFound(Box<str>),
}
