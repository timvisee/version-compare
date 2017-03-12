/// Comparison operators.
#[derive(Debug, Clone, PartialEq)]
pub enum CompOp {
    /// Equal to. (==)
    EQ,

    /// Not equal to. (!=)
    NE,

    /// Less than. (<)
    LT,

    /// Less than or equal to. (<=)
    LE,

    /// Greater than or equal to. (>=)
    GE,

    /// Greater than. (>)
    GT
}

impl CompOp {

    /// Covert to the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GE
    /// - LE <-> GT
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(CompOp::EQ.as_inverted(), CompOp::NE);
    /// assert_eq!(CompOp::LT.as_inverted(), CompOp::GE);
    /// assert_eq!(CompOp::GT.as_inverted(), CompOp::LE);
    /// ```
    pub fn as_inverted(self) -> Self {
        self.invert()
    }

    /// Get the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GE
    /// - LE <-> GT
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(CompOp::EQ.invert(), CompOp::NE);
    /// assert_eq!(CompOp::LT.invert(), CompOp::GE);
    /// assert_eq!(CompOp::GT.invert(), CompOp::LE);
    /// ```
    pub fn invert(&self) -> Self {
        match self {
            &CompOp::EQ => CompOp::NE,
            &CompOp::NE => CompOp::EQ,
            &CompOp::LT => CompOp::GE,
            &CompOp::LE => CompOp::GT,
            &CompOp::GE => CompOp::LT,
            &CompOp::GT => CompOp::LE
        }
    }

    /// Convert to the opposite comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GT
    /// - LE <-> GE
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(CompOp::EQ.as_opposite(), CompOp::NE);
    /// assert_eq!(CompOp::LT.as_opposite(), CompOp::GT);
    /// assert_eq!(CompOp::GE.as_opposite(), CompOp::LE);
    /// ```
    pub fn as_opposite(self) -> Self {
        self.opposite()
    }

    /// Get the opposite comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GT
    /// - LE <-> GE
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(CompOp::EQ.opposite(), CompOp::NE);
    /// assert_eq!(CompOp::LT.opposite(), CompOp::GT);
    /// assert_eq!(CompOp::GE.opposite(), CompOp::LE);
    /// ```
    pub fn opposite(&self) -> Self {
        match self {
            &CompOp::EQ => CompOp::NE,
            &CompOp::NE => CompOp::EQ,
            &CompOp::LT => CompOp::GT,
            &CompOp::LE => CompOp::GE,
            &CompOp::GE => CompOp::LE,
            &CompOp::GT => CompOp::LT
        }
    }

    /// Convert to the flipped comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - LT <-> GT
    /// - LE <-> GE
    /// - Other operators are returned as is.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(CompOp::EQ.as_rotated(), CompOp::EQ);
    /// assert_eq!(CompOp::LT.as_rotated(), CompOp::GT);
    /// assert_eq!(CompOp::GE.as_rotated(), CompOp::LE);
    /// ```
    pub fn as_flipped(self) -> Self {
        self.flip()
    }

    /// Get the flipped comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - LT <-> GT
    /// - LE <-> GE
    /// - Other operators are returned as is.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(CompOp::EQ.rotate(), CompOp::EQ);
    /// assert_eq!(CompOp::LT.rotate(), CompOp::GT);
    /// assert_eq!(CompOp::GE.rotate(), CompOp::LE);
    /// ```
    pub fn flip(&self) -> Self {
        match self {
            &CompOp::LT => CompOp::GT,
            &CompOp::LE => CompOp::GE,
            &CompOp::GE => CompOp::LE,
            &CompOp::GT => CompOp::LT,
            _ => self.clone()
        }
    }

    /// Get the sign for this comparison operator.
    ///
    /// The following signs are returned:
    /// - EQ: `==`
    /// - NE: `!=`
    /// - LT: `<`
    /// - LE: `<=`
    /// - GE: `>=`
    /// - GT: `>`
    pub fn sign(&self) -> &'static str {
        match self {
            &CompOp::EQ => "==",
            &CompOp::NE => "!=",
            &CompOp::LT => "<",
            &CompOp::LE => "<=",
            &CompOp::GE => ">=",
            &CompOp::GT => ">"
        }
    }
}

#[cfg(test)]
mod tests {
    use comp_op::CompOp;

    #[test]
    fn as_inverted() {
        assert_eq!(CompOp::EQ.as_inverted(), CompOp::NE);
        assert_eq!(CompOp::NE.as_inverted(), CompOp::EQ);
        assert_eq!(CompOp::LT.as_inverted(), CompOp::GE);
        assert_eq!(CompOp::LE.as_inverted(), CompOp::GT);
        assert_eq!(CompOp::GE.as_inverted(), CompOp::LT);
        assert_eq!(CompOp::GT.as_inverted(), CompOp::LE);
    }

    #[test]
    fn invert() {
        assert_eq!(CompOp::EQ.invert(), CompOp::NE);
        assert_eq!(CompOp::NE.invert(), CompOp::EQ);
        assert_eq!(CompOp::LT.invert(), CompOp::GE);
        assert_eq!(CompOp::LE.invert(), CompOp::GT);
        assert_eq!(CompOp::GE.invert(), CompOp::LT);
        assert_eq!(CompOp::GT.invert(), CompOp::LE);
    }

    #[test]
    fn as_opposite() {
        assert_eq!(CompOp::EQ.as_opposite(), CompOp::NE);
        assert_eq!(CompOp::NE.as_opposite(), CompOp::EQ);
        assert_eq!(CompOp::LT.as_opposite(), CompOp::GT);
        assert_eq!(CompOp::LE.as_opposite(), CompOp::GE);
        assert_eq!(CompOp::GE.as_opposite(), CompOp::LE);
        assert_eq!(CompOp::GT.as_opposite(), CompOp::LT);
    }

    #[test]
    fn opposite() {
        assert_eq!(CompOp::EQ.opposite(), CompOp::NE);
        assert_eq!(CompOp::NE.opposite(), CompOp::EQ);
        assert_eq!(CompOp::LT.opposite(), CompOp::GT);
        assert_eq!(CompOp::LE.opposite(), CompOp::GE);
        assert_eq!(CompOp::GE.opposite(), CompOp::LE);
        assert_eq!(CompOp::GT.opposite(), CompOp::LT);
    }

    #[test]
    fn as_flipped() {
        assert_eq!(CompOp::EQ.as_flipped(), CompOp::EQ);
        assert_eq!(CompOp::NE.as_flipped(), CompOp::NE);
        assert_eq!(CompOp::LT.as_flipped(), CompOp::GT);
        assert_eq!(CompOp::LE.as_flipped(), CompOp::GE);
        assert_eq!(CompOp::GE.as_flipped(), CompOp::LE);
        assert_eq!(CompOp::GT.as_flipped(), CompOp::LT);
    }

    #[test]
    fn flip() {
        assert_eq!(CompOp::EQ.flip(), CompOp::EQ);
        assert_eq!(CompOp::NE.flip(), CompOp::NE);
        assert_eq!(CompOp::LT.flip(), CompOp::GT);
        assert_eq!(CompOp::LE.flip(), CompOp::GE);
        assert_eq!(CompOp::GE.flip(), CompOp::LE);
        assert_eq!(CompOp::GT.flip(), CompOp::LT);
    }

    #[test]
    fn sign() {
        assert_eq!(CompOp::EQ.sign(), "==");
        assert_eq!(CompOp::NE.sign(), "!=");
        assert_eq!(CompOp::LT.sign(), "<");
        assert_eq!(CompOp::LE.sign(), "<=");
        assert_eq!(CompOp::GE.sign(), ">=");
        assert_eq!(CompOp::GT.sign(), ">");
    }
}