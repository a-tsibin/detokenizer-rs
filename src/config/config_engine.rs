use crate::config::config::CertConfig;

struct ConfigEngine<'a> {
    config: &'a CertConfig
}

impl<'a> ConfigEngine<'a> {
    fn create(cert_config: &CertConfig) -> ConfigEngine {
        ConfigEngine { config: cert_config }
    }
}