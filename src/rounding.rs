/// Rounding methods for metal values.
#[derive(Debug, PartialEq, Clone)]
pub enum Rounding {
    /// Rounds up to the nearest scrap.
    UpScrap,
    /// Rounds down to the nearest scrap.
    DownScrap,
    /// Rounds to the nearest refined.
    Refined,
    /// Rounds up to the nearest refined.
    UpRefined,
    /// Rounds up to the nearest refined.
    DownRefined,
    /// No rounding.
    None,
}