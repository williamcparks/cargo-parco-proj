use std::{collections::HashSet, ops::Deref};

use serde::Deserialize;
use toml::{Table, Value};

use super::error::ProjError;

#[derive(Deserialize)]
pub struct Proj {
    pub config: Config,
    pub env: Env,
}

#[derive(Deserialize)]
pub struct Config {
    pub setting: Setting,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Setting {
    Dev,
    Prod,
}

#[derive(Deserialize)]
pub struct Env {
    pub base: Table,
    pub dev: Table,
    pub prod: Table,
}

impl Proj {
    pub fn try_new(contents: &str) -> Result<Self, ProjError> {
        let proj: Self = toml::from_str(contents)?;

        let base = set(&proj.env.base);
        let dev = set(&proj.env.dev);
        let prod = set(&proj.env.prod);
        if let Some(key) = base.intersection(&dev).next() {
            Err(ProjError::BaseOverWriteDev((*key).into()))
        } else if let Some(key) = base.intersection(&prod).next() {
            Err(ProjError::BaseOverWriteProd((*key).into()))
        } else if let Some(key) = dev.difference(&prod).next() {
            Err(ProjError::KeyInDevNotInProd((*key).into()))
        } else if let Some(key) = prod.difference(&dev).next() {
            Err(ProjError::KeyInProdNotInDev((*key).into()))
        } else {
            Ok(proj)
        }
    }

    pub fn try_get<'a>(&'a self, key: &str) -> Result<&'a Value, ProjError> {
        if let Some(some) = self.env.base.get(key) {
            Ok(some)
        } else {
            let map = match self.config.setting {
                Setting::Dev => &self.env.dev,
                Setting::Prod => &self.env.prod,
            };

            match map.get(key) {
                Some(some) => Ok(some),
                None => Err(ProjError::KeyNotFound(key.into())),
            }
        }
    }
}

fn set(table: &Table) -> HashSet<&str> {
    table.keys().map(Deref::deref).collect()
}
