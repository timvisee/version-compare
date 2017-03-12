/// Comparison operators.
pub enum CompOp {
    /// Equal to. (==)
    EQ,

    /// Not equal to. (!=)
    NE,

    /// Less than. (<)
    LT,

    /// Less than or equal to. (<=)
    LE,

    /// Greater than. (>)
    GT,

    /// Greater than or equal to. (>=)
    GE,

}

impl CompOp {

    /// Covert to the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GE
    /// - LE <-> GT
    pub fn as_inverted(self) -> Self {
        self.invert()
    }

    /// Get the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GE
    /// - LE <-> GT
    pub fn invert(&self) -> Self {
        match self {
            &CompOp::EQ => CompOp::NE,
            &CompOp::NE => CompOp::EQ,
            &CompOp::LT => CompOp::GE,
            &CompOp::LE => CompOp::GT,
            &CompOp::GT => CompOp::LE,
            &CompOp::GE => CompOp::LT
        }
    }

    /// Convert to the opposite comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GT
    /// - LE <-> GE
    pub fn as_opposite(self) -> Self {
        self.opposite()
    }

    /// Get the opposite comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GT
    /// - LE <-> GE
    pub fn opposite(&self) -> Self {
        match self {
            &CompOp::EQ => CompOp::NE,
            &CompOp::NE => CompOp::EQ,
            &CompOp::LT => CompOp::GT,
            &CompOp::LE => CompOp::GE,
            &CompOp::GT => CompOp::LT,
            &CompOp::GE => CompOp::LE
        }
    }
}