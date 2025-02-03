use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum TileFlipValues {
    Horizontal,
    HorizontalAndVertical,
    None,
    Vertical,
}
impl Default for TileFlipValues {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for TileFlipValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::Horizontal => "x",
            Self::HorizontalAndVertical => "xy",
            Self::None => "none",
            Self::Vertical => "y",
        }
    }
}
impl FromStr for TileFlipValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "x" => Ok(Self::Horizontal),
            "xy" => Ok(Self::HorizontalAndVertical),
            "none" => Ok(Self::None),
            "y" => Ok(Self::Vertical),
            _ => Err(()),
        }
    }
}
