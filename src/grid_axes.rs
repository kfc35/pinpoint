/// Contains the axes for a given day of Pinpoint.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Axes {
    horizontal: AxisSpectrum,
    vertical: AxisSpectrum,
}

impl Axes {
    pub(crate) fn horizontal(&self) -> AxisSpectrum {
        self.horizontal
    }

    pub(crate) fn vertical(&self) -> AxisSpectrum {
        self.vertical
    }
}

/// Holds a possible axis that can be used for a given game.
/// The first entry is left/up. The right entry is right/bottom.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct AxisSpectrum(&'static str, &'static str);

impl AxisSpectrum {
    pub(crate) fn first(&self) -> &'static str {
        self.0
    }

    pub(crate) fn second(&self) -> &'static str {
        self.1
    }
}

/// Returns the Axes for a given day.
pub(crate) fn get_axes(_date: &String) -> Axes {
    AXES[0]
}

const AXIS_SPECTRA: [AxisSpectrum; 10] = [
    // Seasons
    AxisSpectrum("Spring-y", "Summer-y"),
    AxisSpectrum("Autumnal", "Wintry"),
    // Directions
    AxisSpectrum("Western", "Eastern"),
    AxisSpectrum("Northern", "Southern"),
    // US Cities
    AxisSpectrum("NYC", "BOS"),
    AxisSpectrum("SF", "LA"),
    // Knowledge
    AxisSpectrum("Obscure", "Well Known"),
    AxisSpectrum("Important", "Insignificant"),
    // Moments
    AxisSpectrum("Unpleasant", "Enjoyable"),
    AxisSpectrum("Rare", "Common"),
];

const AXES: [Axes; 5] = [
    Axes {
        horizontal: AXIS_SPECTRA[0],
        vertical: AXIS_SPECTRA[1],
    },
    Axes {
        horizontal: AXIS_SPECTRA[2],
        vertical: AXIS_SPECTRA[3],
    },
    Axes {
        horizontal: AXIS_SPECTRA[4],
        vertical: AXIS_SPECTRA[5],
    },
    Axes {
        horizontal: AXIS_SPECTRA[6],
        vertical: AXIS_SPECTRA[7],
    },
    Axes {
        horizontal: AXIS_SPECTRA[8],
        vertical: AXIS_SPECTRA[9],
    },
];
