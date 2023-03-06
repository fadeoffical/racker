use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManifest {
    name: String,
    version: Option<String>,
    description: Option<String>,
    license: Option<String>,
    authors: Option<Vec<Author>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    name: String,
    file: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    name: String,
    email: Option<String>,
    socials: Option<Socials>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socials {
    github: Option<String>,
    discord: Option<String>,
}

impl Socials {
    /// Returns the github url of the author.
    pub fn github(&self) -> Option<String> {
        self.github.as_ref().map(|github| String::from("https://github.com/") + github)
    }

    /// Returns the discord name and discriminator of the author.
    pub fn discord(&self) -> Option<String> {
        self.discord.as_ref().cloned()
    }
}

impl Default for Socials {
    /// Creates a new instance of `Socials`.
    fn default() -> Self {
        Self {
            github: None,
            discord: None,
        }
    }
}
