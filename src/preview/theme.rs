use std::fmt;
use std::str::FromStr;

/// A daisyUI theme the Preview can be viewed under.
///
/// The list has to stay in step with `tailwind.css`, which asks for
/// `themes: all`: daisyUI emits custom properties only for the themes named
/// there, and a theme it never emitted leaves the page on the one it was
/// already under. That is also what makes the theme controller's Examples
/// inert (ADR-0020): they name themes daisyUI does not ship.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Theme {
    name: &'static str,
    title: &'static str,
    baseline: bool,
}

impl Theme {
    /// Every theme the Preview offers, in daisyUI's order.
    ///
    /// Baseline themes are marked in the same entry rather than repeated in a
    /// second list. The three are the default, the theme controller's dark
    /// side, and one that moves radii and borders as well as colours.
    pub const ALL: &'static [Self] = &[
        Self::new("light", "Light", true),
        Self::new("dark", "Dark", true),
        Self::new("cupcake", "Cupcake", true),
        Self::new("bumblebee", "Bumblebee", false),
        Self::new("emerald", "Emerald", false),
        Self::new("corporate", "Corporate", false),
        Self::new("synthwave", "Synthwave", false),
        Self::new("retro", "Retro", false),
        Self::new("cyberpunk", "Cyberpunk", false),
        Self::new("valentine", "Valentine", false),
        Self::new("halloween", "Halloween", false),
        Self::new("garden", "Garden", false),
        Self::new("forest", "Forest", false),
        Self::new("aqua", "Aqua", false),
        Self::new("lofi", "Lo-fi", false),
        Self::new("pastel", "Pastel", false),
        Self::new("fantasy", "Fantasy", false),
        Self::new("wireframe", "Wireframe", false),
        Self::new("black", "Black", false),
        Self::new("luxury", "Luxury", false),
        Self::new("dracula", "Dracula", false),
        Self::new("cmyk", "CMYK", false),
        Self::new("autumn", "Autumn", false),
        Self::new("business", "Business", false),
        Self::new("acid", "Acid", false),
        Self::new("lemonade", "Lemonade", false),
        Self::new("night", "Night", false),
        Self::new("coffee", "Coffee", false),
        Self::new("winter", "Winter", false),
        Self::new("dim", "Dim", false),
        Self::new("nord", "Nord", false),
        Self::new("sunset", "Sunset", false),
        Self::new("caramellatte", "Caramellatte", false),
        Self::new("abyss", "Abyss", false),
        Self::new("silk", "Silk", false),
    ];

    const fn new(name: &'static str, title: &'static str, baseline: bool) -> Self {
        Self {
            name,
            title,
            baseline,
        }
    }

    /// The name daisyUI matches and the URL uses.
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The label the switcher displays.
    pub const fn title(self) -> &'static str {
        self.title
    }

    /// Whether browser tests take a screenshot baseline under this theme.
    pub const fn is_baseline(self) -> bool {
        self.baseline
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::ALL[0]
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// The error [`Theme::from_str`] returns for a name no theme goes by.
#[derive(Debug)]
pub struct UnknownTheme(String);

impl fmt::Display for UnknownTheme {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "no theme is named '{}'", self.0)
    }
}

impl FromStr for Theme {
    type Err = UnknownTheme;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .copied()
            .find(|theme| theme.name() == name)
            .ok_or_else(|| UnknownTheme(name.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_baseline_themes_are_declared_in_the_catalog() {
        assert_eq!(Theme::default().name(), "light");
        assert_eq!(
            Theme::ALL
                .iter()
                .copied()
                .filter(|theme| theme.is_baseline())
                .map(Theme::name)
                .collect::<Vec<_>>(),
            ["light", "dark", "cupcake"]
        );
    }

    #[test]
    fn parsing_uses_theme_names() {
        assert_eq!(Theme::from_str("nord").unwrap().title(), "Nord");
        assert!(Theme::from_str("unknown").is_err());
    }
}
