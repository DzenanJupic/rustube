use anyhow::Result;

#[derive(Clone, Copy, Debug, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum OutputFormat {
    Debug,
    PrettyDebug,
    Json,
    PrettyJson,
    Yaml,
}

impl OutputFormat {
    pub fn serialize_output<T>(&self, output: &T) -> Result<String>
        where T: serde::Serialize + std::fmt::Debug {
        use OutputFormat::*;

        match self {
            Debug => Ok(format!("{:?}", output)),
            PrettyDebug => Ok(format!("{:#?}", output)),
            Json => Ok(serde_json::to_string(output)?),
            PrettyJson => Ok(serde_json::to_string_pretty(output)?),
            Yaml => Ok(serde_yaml::to_string(output)?)
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::PrettyJson
    }
}
