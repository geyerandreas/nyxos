use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OAuth2 {
    /// Enable or disable OAuth2 authentication.
    pub enabled: bool,

    /// OIDC issuer URL for the OAuth2 provider.
    pub issuer_url: Option<String>,

    /// The client ID for the OAuth2 provider.
    pub client_id: Option<String>,

    /// The client secret for the OAuth2 provider.
    #[serde(skip_serializing)]
    pub client_secret: Option<String>,

    /// The scopes to request from the OAuth2 provider.
    pub scopes: Vec<String>,
}

impl Default for OAuth2 {
    fn default() -> Self {
        OAuth2 {
            enabled: false,
            issuer_url: None,
            client_id: None,
            client_secret: None,
            scopes: default_scopes(),
        }
    }
}

fn default_scopes() -> Vec<String> {
    vec![
        "openid".to_owned(),
        "profile".to_owned(),
        "email".to_owned(),
    ]
}

impl OAuth2 {
    /// Validates the OAuth2 configuration.
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        if self.issuer_url.is_none() {
            return Err("Issuer URL is required when OAuth2 is enabled".to_string());
        }

        if self.client_id.is_none() {
            return Err("Client ID is required when OAuth2 is enabled".to_string());
        }

        if self.client_secret.is_none() {
            return Err("Client secret is required when OAuth2 is enabled".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings_are_valid() {
        let settings = OAuth2::default();
        assert!(settings.validate().is_ok());
    }
}
