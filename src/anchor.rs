use crate::error::*;

/// Where to anchor the panel
#[allow(missing_docs)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl std::str::FromStr for Anchor {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "top-left" => Anchor::TopLeft,
            "top-right" => Anchor::TopRight,
            "bottom-left" => Anchor::BottomLeft,
            "bottom-right" => Anchor::BottomRight,
            s => {
                return Err(ErrorKind::ConfigError(format!(
                    "Uncrecognized anchor: {}",
                    s
                ))
                .into());
            }
        })
    }
}
