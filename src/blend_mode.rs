use crate::pipeline;

/// A blending mode.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub enum BlendMode {
    /// Replaces destination with zero: fully transparent.
    Clear,
    /// Replaces destination.
    Source,
    /// Preserves destination.
    Destination,
    /// Source over destination.
    #[default]
    SourceOver,
    /// Destination over source.
    DestinationOver,
    /// Source trimmed inside destination.
    SourceIn,
    /// Destination trimmed by source.
    DestinationIn,
    /// Source trimmed outside destination.
    SourceOut,
    /// Destination trimmed outside source.
    DestinationOut,
    /// Source inside destination blended with destination.
    SourceAtop,
    /// Destination inside source blended with source.
    DestinationAtop,
    /// Each of source and destination trimmed outside the other.
    Xor,
    /// Sum of colors.
    Plus,
    /// Product of premultiplied colors; darkens destination.
    Modulate,
    /// Multiply inverse of pixels, inverting result; brightens destination.
    Screen,
    /// Multiply or screen, depending on destination.
    Overlay,
    /// Darker of source and destination.
    Darken,
    /// Lighter of source and destination.
    Lighten,
    /// Brighten destination to reflect source.
    ColorDodge,
    /// Darken destination to reflect source.
    ColorBurn,
    /// Multiply or screen, depending on source.
    HardLight,
    /// Lighten or darken, depending on source.
    SoftLight,
    /// Subtract darker from lighter with higher contrast.
    Difference,
    /// Subtract darker from lighter with lower contrast.
    Exclusion,
    /// Multiply source with destination, darkening image.
    Multiply,
    /// Hue of source with saturation and luminosity of destination.
    Hue,
    /// Saturation of source with hue and luminosity of destination.
    Saturation,
    /// Hue and saturation of source with luminosity of destination.
    Color,
    /// Luminosity of source with hue and saturation of destination.
    Luminosity,
}

impl BlendMode {
    pub(crate) const fn should_pre_scale_coverage(self) -> bool {
        // The most important things we do here are:
        //   1) never pre-scale with rgb coverage if the blend mode involves a source-alpha term;
        //   2) always pre-scale Plus.
        //
        // When we pre-scale with rgb coverage, we scale each of source r,g,b, with a distinct value,
        // and source alpha with one of those three values. This process destructively updates the
        // source-alpha term, so we can't evaluate blend modes that need its original value.
        //
        // Plus always requires pre-scaling as a specific quirk of its implementation in
        // RasterPipeline. This lets us put the clamp inside the blend mode itself rather
        // than as a separate stage that'd come after the lerp.
        //
        // This function is a finer-grained breakdown of SkBlendMode_SupportsCoverageAsAlpha().
        matches!(
            self,
            Self::Destination |        // d              --> no sa term, ok!
            Self::DestinationOver |    // d + s*inv(da)  --> no sa term, ok!
            Self::Plus |               // clamp(s+d)     --> no sa term, ok!
            Self::DestinationOut |     // d * inv(sa)
            Self::SourceAtop |         // s*da + d*inv(sa)
            Self::SourceOver |         // s + d*inv(sa)
            Self::Xor // s*inv(da) + d*inv(sa)
        )
    }

    pub(crate) const fn to_stage(self) -> Option<pipeline::Stage> {
        match self {
            Self::Clear => Some(pipeline::Stage::Clear),
            Self::Source => None, // This stage is a no-op.
            Self::Destination => Some(pipeline::Stage::MoveDestinationToSource),
            Self::SourceOver => Some(pipeline::Stage::SourceOver),
            Self::DestinationOver => Some(pipeline::Stage::DestinationOver),
            Self::SourceIn => Some(pipeline::Stage::SourceIn),
            Self::DestinationIn => Some(pipeline::Stage::DestinationIn),
            Self::SourceOut => Some(pipeline::Stage::SourceOut),
            Self::DestinationOut => Some(pipeline::Stage::DestinationOut),
            Self::SourceAtop => Some(pipeline::Stage::SourceAtop),
            Self::DestinationAtop => Some(pipeline::Stage::DestinationAtop),
            Self::Xor => Some(pipeline::Stage::Xor),
            Self::Plus => Some(pipeline::Stage::Plus),
            Self::Modulate => Some(pipeline::Stage::Modulate),
            Self::Screen => Some(pipeline::Stage::Screen),
            Self::Overlay => Some(pipeline::Stage::Overlay),
            Self::Darken => Some(pipeline::Stage::Darken),
            Self::Lighten => Some(pipeline::Stage::Lighten),
            Self::ColorDodge => Some(pipeline::Stage::ColorDodge),
            Self::ColorBurn => Some(pipeline::Stage::ColorBurn),
            Self::HardLight => Some(pipeline::Stage::HardLight),
            Self::SoftLight => Some(pipeline::Stage::SoftLight),
            Self::Difference => Some(pipeline::Stage::Difference),
            Self::Exclusion => Some(pipeline::Stage::Exclusion),
            Self::Multiply => Some(pipeline::Stage::Multiply),
            Self::Hue => Some(pipeline::Stage::Hue),
            Self::Saturation => Some(pipeline::Stage::Saturation),
            Self::Color => Some(pipeline::Stage::Color),
            Self::Luminosity => Some(pipeline::Stage::Luminosity),
        }
    }
}
