use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum PresetCameraValues {
    IsometricBottomDown,
    IsometricBottomUp,
    IsometricLeftDown,
    IsometricLeftUp,
    IsometricOffAxis1Left,
    IsometricOffAxis1Right,
    IsometricOffAxis1Top,
    IsometricOffAxis2Left,
    IsometricOffAxis2Right,
    IsometricOffAxis2Top,
    IsometricOffAxis3Bottom,
    IsometricOffAxis3Left,
    IsometricOffAxis3Right,
    IsometricOffAxis4Bottom,
    IsometricOffAxis4Left,
    IsometricOffAxis4Right,
    IsometricRightDown,
    IsometricRightUp,
    IsometricTopDown,
    IsometricTopUp,
    LegacyObliqueBottom,
    LegacyObliqueBottomLeft,
    LegacyObliqueBottomRight,
    LegacyObliqueFront,
    LegacyObliqueLeft,
    LegacyObliqueRight,
    LegacyObliqueTop,
    LegacyObliqueTopLeft,
    LegacyObliqueTopRight,
    LegacyPerspectiveBottom,
    LegacyPerspectiveBottomLeft,
    LegacyPerspectiveBottomRight,
    LegacyPerspectiveFront,
    LegacyPerspectiveLeft,
    LegacyPerspectiveRight,
    LegacyPerspectiveTop,
    LegacyPerspectiveTopLeft,
    LegacyPerspectiveTopRight,
    ObliqueBottom,
    ObliqueBottomLeft,
    ObliqueBottomRight,
    ObliqueLeft,
    ObliqueRight,
    ObliqueTop,
    ObliqueTopLeft,
    ObliqueTopRight,
    OrthographicFront,
    PerspectiveAbove,
    PerspectiveAboveLeftFacing,
    PerspectiveAboveRightFacing,
    PerspectiveBelow,
    PerspectiveContrastingLeftFacing,
    PerspectiveContrastingRightFacing,
    PerspectiveFront,
    PerspectiveHeroicExtremeLeftFacing,
    PerspectiveHeroicExtremeRightFacing,
    PerspectiveHeroicLeftFacing,
    PerspectiveHeroicRightFacing,
    PerspectiveLeft,
    PerspectiveRelaxed,
    PerspectiveRelaxedModerately,
    PerspectiveRight,
}
impl Default for PresetCameraValues {
    #[inline]
    fn default() -> Self {
        Self::LegacyObliqueTopLeft
    }
}
impl EnumTrait for PresetCameraValues {
    #[inline]
    fn get_value_string(&self) -> &str {
        match &self {
            Self::IsometricBottomDown => "isometricBottomDown",
            Self::IsometricBottomUp => "isometricBottomUp",
            Self::IsometricLeftDown => "isometricLeftDown",
            Self::IsometricLeftUp => "isometricLeftUp",
            Self::IsometricOffAxis1Left => "isometricOffAxis1Left",
            Self::IsometricOffAxis1Right => "isometricOffAxis1Right",
            Self::IsometricOffAxis1Top => "isometricOffAxis1Top",
            Self::IsometricOffAxis2Left => "isometricOffAxis2Left",
            Self::IsometricOffAxis2Right => "isometricOffAxis2Right",
            Self::IsometricOffAxis2Top => "isometricOffAxis2Top",
            Self::IsometricOffAxis3Bottom => "isometricOffAxis3Bottom",
            Self::IsometricOffAxis3Left => "isometricOffAxis3Left",
            Self::IsometricOffAxis3Right => "isometricOffAxis3Right",
            Self::IsometricOffAxis4Bottom => "isometricOffAxis4Bottom",
            Self::IsometricOffAxis4Left => "isometricOffAxis4Left",
            Self::IsometricOffAxis4Right => "isometricOffAxis4Right",
            Self::IsometricRightDown => "isometricRightDown",
            Self::IsometricRightUp => "isometricRightUp",
            Self::IsometricTopDown => "isometricTopDown",
            Self::IsometricTopUp => "isometricTopUp",
            Self::LegacyObliqueBottom => "legacyObliqueBottom",
            Self::LegacyObliqueBottomLeft => "legacyObliqueBottomLeft",
            Self::LegacyObliqueBottomRight => "legacyObliqueBottomRight",
            Self::LegacyObliqueFront => "legacyObliqueFront",
            Self::LegacyObliqueLeft => "legacyObliqueLeft",
            Self::LegacyObliqueRight => "legacyObliqueRight",
            Self::LegacyObliqueTop => "legacyObliqueTop",
            Self::LegacyObliqueTopLeft => "legacyObliqueTopLeft",
            Self::LegacyObliqueTopRight => "legacyObliqueTopRight",
            Self::LegacyPerspectiveBottom => "legacyPerspectiveBottom",
            Self::LegacyPerspectiveBottomLeft => "legacyPerspectiveBottomLeft",
            Self::LegacyPerspectiveBottomRight => "legacyPerspectiveBottomRight",
            Self::LegacyPerspectiveFront => "legacyPerspectiveFront",
            Self::LegacyPerspectiveLeft => "legacyPerspectiveLeft",
            Self::LegacyPerspectiveRight => "legacyPerspectiveRight",
            Self::LegacyPerspectiveTop => "legacyPerspectiveTop",
            Self::LegacyPerspectiveTopLeft => "legacyPerspectiveTopLeft",
            Self::LegacyPerspectiveTopRight => "legacyPerspectiveTopRight",
            Self::ObliqueBottom => "obliqueBottom",
            Self::ObliqueBottomLeft => "obliqueBottomLeft",
            Self::ObliqueBottomRight => "obliqueBottomRight",
            Self::ObliqueLeft => "obliqueLeft",
            Self::ObliqueRight => "obliqueRight",
            Self::ObliqueTop => "obliqueTop",
            Self::ObliqueTopLeft => "obliqueTopLeft",
            Self::ObliqueTopRight => "obliqueTopRight",
            Self::OrthographicFront => "orthographicFront",
            Self::PerspectiveAbove => "perspectiveAbove",
            Self::PerspectiveAboveLeftFacing => "perspectiveAboveLeftFacing",
            Self::PerspectiveAboveRightFacing => "perspectiveAboveRightFacing",
            Self::PerspectiveBelow => "perspectiveBelow",
            Self::PerspectiveContrastingLeftFacing => "perspectiveContrastingLeftFacing",
            Self::PerspectiveContrastingRightFacing => "perspectiveContrastingRightFacing",
            Self::PerspectiveFront => "perspectiveFront",
            Self::PerspectiveHeroicExtremeLeftFacing => "perspectiveHeroicExtremeLeftFacing",
            Self::PerspectiveHeroicExtremeRightFacing => "perspectiveHeroicExtremeRightFacing",
            Self::PerspectiveHeroicLeftFacing => "perspectiveHeroicLeftFacing",
            Self::PerspectiveHeroicRightFacing => "perspectiveHeroicRightFacing",
            Self::PerspectiveLeft => "perspectiveLeft",
            Self::PerspectiveRelaxed => "perspectiveRelaxed",
            Self::PerspectiveRelaxedModerately => "perspectiveRelaxedModerately",
            Self::PerspectiveRight => "perspectiveRight",
        }
    }
}
impl FromStr for PresetCameraValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "isometricBottomDown" => Ok(Self::IsometricBottomDown),
            "isometricBottomUp" => Ok(Self::IsometricBottomUp),
            "isometricLeftDown" => Ok(Self::IsometricLeftDown),
            "isometricLeftUp" => Ok(Self::IsometricLeftUp),
            "isometricOffAxis1Left" => Ok(Self::IsometricOffAxis1Left),
            "isometricOffAxis1Right" => Ok(Self::IsometricOffAxis1Right),
            "isometricOffAxis1Top" => Ok(Self::IsometricOffAxis1Top),
            "isometricOffAxis2Left" => Ok(Self::IsometricOffAxis2Left),
            "isometricOffAxis2Right" => Ok(Self::IsometricOffAxis2Right),
            "isometricOffAxis2Top" => Ok(Self::IsometricOffAxis2Top),
            "isometricOffAxis3Bottom" => Ok(Self::IsometricOffAxis3Bottom),
            "isometricOffAxis3Left" => Ok(Self::IsometricOffAxis3Left),
            "isometricOffAxis3Right" => Ok(Self::IsometricOffAxis3Right),
            "isometricOffAxis4Bottom" => Ok(Self::IsometricOffAxis4Bottom),
            "isometricOffAxis4Left" => Ok(Self::IsometricOffAxis4Left),
            "isometricOffAxis4Right" => Ok(Self::IsometricOffAxis4Right),
            "isometricRightDown" => Ok(Self::IsometricRightDown),
            "isometricRightUp" => Ok(Self::IsometricRightUp),
            "isometricTopDown" => Ok(Self::IsometricTopDown),
            "isometricTopUp" => Ok(Self::IsometricTopUp),
            "legacyObliqueBottom" => Ok(Self::LegacyObliqueBottom),
            "legacyObliqueBottomLeft" => Ok(Self::LegacyObliqueBottomLeft),
            "legacyObliqueBottomRight" => Ok(Self::LegacyObliqueBottomRight),
            "legacyObliqueFront" => Ok(Self::LegacyObliqueFront),
            "legacyObliqueLeft" => Ok(Self::LegacyObliqueLeft),
            "legacyObliqueRight" => Ok(Self::LegacyObliqueRight),
            "legacyObliqueTop" => Ok(Self::LegacyObliqueTop),
            "legacyObliqueTopLeft" => Ok(Self::LegacyObliqueTopLeft),
            "legacyObliqueTopRight" => Ok(Self::LegacyObliqueTopRight),
            "legacyPerspectiveBottom" => Ok(Self::LegacyPerspectiveBottom),
            "legacyPerspectiveBottomLeft" => Ok(Self::LegacyPerspectiveBottomLeft),
            "legacyPerspectiveBottomRight" => Ok(Self::LegacyPerspectiveBottomRight),
            "legacyPerspectiveFront" => Ok(Self::LegacyPerspectiveFront),
            "legacyPerspectiveLeft" => Ok(Self::LegacyPerspectiveLeft),
            "legacyPerspectiveRight" => Ok(Self::LegacyPerspectiveRight),
            "legacyPerspectiveTop" => Ok(Self::LegacyPerspectiveTop),
            "legacyPerspectiveTopLeft" => Ok(Self::LegacyPerspectiveTopLeft),
            "legacyPerspectiveTopRight" => Ok(Self::LegacyPerspectiveTopRight),
            "obliqueBottom" => Ok(Self::ObliqueBottom),
            "obliqueBottomLeft" => Ok(Self::ObliqueBottomLeft),
            "obliqueBottomRight" => Ok(Self::ObliqueBottomRight),
            "obliqueLeft" => Ok(Self::ObliqueLeft),
            "obliqueRight" => Ok(Self::ObliqueRight),
            "obliqueTop" => Ok(Self::ObliqueTop),
            "obliqueTopLeft" => Ok(Self::ObliqueTopLeft),
            "obliqueTopRight" => Ok(Self::ObliqueTopRight),
            "orthographicFront" => Ok(Self::OrthographicFront),
            "perspectiveAbove" => Ok(Self::PerspectiveAbove),
            "perspectiveAboveLeftFacing" => Ok(Self::PerspectiveAboveLeftFacing),
            "perspectiveAboveRightFacing" => Ok(Self::PerspectiveAboveRightFacing),
            "perspectiveBelow" => Ok(Self::PerspectiveBelow),
            "perspectiveContrastingLeftFacing" => Ok(Self::PerspectiveContrastingLeftFacing),
            "perspectiveContrastingRightFacing" => Ok(Self::PerspectiveContrastingRightFacing),
            "perspectiveFront" => Ok(Self::PerspectiveFront),
            "perspectiveHeroicExtremeLeftFacing" => Ok(Self::PerspectiveHeroicExtremeLeftFacing),
            "perspectiveHeroicExtremeRightFacing" => Ok(Self::PerspectiveHeroicExtremeRightFacing),
            "perspectiveHeroicLeftFacing" => Ok(Self::PerspectiveHeroicLeftFacing),
            "perspectiveHeroicRightFacing" => Ok(Self::PerspectiveHeroicRightFacing),
            "perspectiveLeft" => Ok(Self::PerspectiveLeft),
            "perspectiveRelaxed" => Ok(Self::PerspectiveRelaxed),
            "perspectiveRelaxedModerately" => Ok(Self::PerspectiveRelaxedModerately),
            "perspectiveRight" => Ok(Self::PerspectiveRight),
            _ => Err(()),
        }
    }
}
