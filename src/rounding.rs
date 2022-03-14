/// Rounding methods for metal values.
#[derive(Debug, PartialEq, Clone)]
pub enum Rounding {
    UpScrap,
    DownScrap,
    Refined,
    UpRefined,
    DownRefined,
    None,
}