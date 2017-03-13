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

    /// Get a comparison operator by it's sign.
    /// Whitespaces are stripped from the sign string.
    /// An error is returned if the sign isn't recognized.
    ///
    /// The following signs are supported:
    /// - ==: `EQ`
    /// - !=: `NE`
    /// - <:  `LT`
    /// - <=: `LE`
    /// - >=: `GE`
    /// - >:  `GT`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::from_sign("=="), Ok(CompOp::EQ));
    /// assert_eq!(CompOp::from_sign("<"), Ok(CompOp::LT));
    /// assert_eq!(CompOp::from_sign("  >=   "), Ok(CompOp::GE));
    /// assert!(CompOp::from_sign("*").is_err());
    /// ```
    pub fn from_sign(sign: &str) -> Result<CompOp, ()> {
        match sign.trim().as_ref() {
            "==" => Ok(CompOp::EQ),
            "!=" => Ok(CompOp::NE),
            "<" => Ok(CompOp::LT),
            "<=" => Ok(CompOp::LE),
            ">=" => Ok(CompOp::GE),
            ">" => Ok(CompOp::GT),
            _ => Err(())
        }
    }

    /// Get a comparison operator by it's name.
    /// Names are case-insensitive, and whitespaces are stripped from the string.
    /// An error is returned if the name isn't recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::from_name("eq"), Ok(CompOp::EQ));
    /// assert_eq!(CompOp::from_name("lt"), Ok(CompOp::LT));
    /// assert_eq!(CompOp::from_name("  Ge   "), Ok(CompOp::GE));
    /// assert!(CompOp::from_name("abc").is_err());
    /// ```
    pub fn from_name(sign: &str) -> Result<CompOp, ()> {
        match sign.trim().to_lowercase().as_ref() {
            "eq" => Ok(CompOp::EQ),
            "ne" => Ok(CompOp::NE),
            "lt" => Ok(CompOp::LT),
            "le" => Ok(CompOp::LE),
            "ge" => Ok(CompOp::GE),
            "gt" => Ok(CompOp::GT),
            _ => Err(())
        }
    }

    /// Covert to the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - EQ <-> NE
    /// - LT <-> GE
    /// - LE <-> GT
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
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
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
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
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
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
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
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
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::EQ.as_flipped(), CompOp::EQ);
    /// assert_eq!(CompOp::LT.as_flipped(), CompOp::GT);
    /// assert_eq!(CompOp::GE.as_flipped(), CompOp::LE);
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
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::EQ.flip(), CompOp::EQ);
    /// assert_eq!(CompOp::LT.flip(), CompOp::GT);
    /// assert_eq!(CompOp::GE.flip(), CompOp::LE);
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
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::EQ.sign(), "==");
    /// assert_eq!(CompOp::LT.sign(), "<");
    /// assert_eq!(CompOp::GE.flip().sign(), "<=");
    /// ```
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

    /// Get a factor (number) for this comparison operator.
    /// These factors can be useful for quick calculations.
    ///
    /// The following factor numbers are returned:
    /// - EQ | NE: `0`
    /// - LT | LE: `-1`
    /// - GT | GE: `1`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::version::Version;
    ///
    /// let ver_a = Version::from("1.2.3").unwrap();
    /// let ver_b = Version::from("1.3").unwrap();
    ///
    /// assert_eq!(ver_a.compare(&ver_b).factor(), -1);
    /// assert_eq!(10 * ver_b.compare(&ver_a).factor(), 10);
    /// ```
    pub fn factor(&self) -> i8 {
        match self {
            &CompOp::EQ | &CompOp::NE => 0,
            &CompOp::LT | &CompOp::LE => -1,
            &CompOp::GT | &CompOp::GE => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use comp_op::CompOp;

    #[test]
    fn from_sign() {
        // Normal signs
        assert_eq!(CompOp::from_sign("==").unwrap(), CompOp::EQ);
        assert_eq!(CompOp::from_sign("!=").unwrap(), CompOp::NE);
        assert_eq!(CompOp::from_sign("<").unwrap(), CompOp::LT);
        assert_eq!(CompOp::from_sign("<=").unwrap(), CompOp::LE);
        assert_eq!(CompOp::from_sign(">=").unwrap(), CompOp::GE);
        assert_eq!(CompOp::from_sign(">").unwrap(), CompOp::GT);

        // Exceptional cases
        assert_eq!(CompOp::from_sign("  <=  ").unwrap(), CompOp::LE);
        assert!(CompOp::from_sign("*").is_err());
    }

    #[test]
    fn from_name() {
        // Normal names
        assert_eq!(CompOp::from_name("eq").unwrap(), CompOp::EQ);
        assert_eq!(CompOp::from_name("ne").unwrap(), CompOp::NE);
        assert_eq!(CompOp::from_name("lt").unwrap(), CompOp::LT);
        assert_eq!(CompOp::from_name("le").unwrap(), CompOp::LE);
        assert_eq!(CompOp::from_name("ge").unwrap(), CompOp::GE);
        assert_eq!(CompOp::from_name("gt").unwrap(), CompOp::GT);

        // Exceptional cases
        assert_eq!(CompOp::from_name("  Le  ").unwrap(), CompOp::LE);
        assert!(CompOp::from_name("abc").is_err());
    }

    #[test]
    fn as_inverted() {
        assert_eq!(CompOp::NE.as_inverted(), CompOp::EQ);
        assert_eq!(CompOp::EQ.as_inverted(), CompOp::NE);
        assert_eq!(CompOp::GE.as_inverted(), CompOp::LT);
        assert_eq!(CompOp::GT.as_inverted(), CompOp::LE);
        assert_eq!(CompOp::LT.as_inverted(), CompOp::GE);
        assert_eq!(CompOp::LE.as_inverted(), CompOp::GT);
    }

    #[test]
    fn invert() {
        assert_eq!(CompOp::NE.invert(), CompOp::EQ);
        assert_eq!(CompOp::EQ.invert(), CompOp::NE);
        assert_eq!(CompOp::GE.invert(), CompOp::LT);
        assert_eq!(CompOp::GT.invert(), CompOp::LE);
        assert_eq!(CompOp::LT.invert(), CompOp::GE);
        assert_eq!(CompOp::LE.invert(), CompOp::GT);
    }

    #[test]
    fn as_opposite() {
        assert_eq!(CompOp::NE.as_opposite(), CompOp::EQ);
        assert_eq!(CompOp::EQ.as_opposite(), CompOp::NE);
        assert_eq!(CompOp::GT.as_opposite(), CompOp::LT);
        assert_eq!(CompOp::GE.as_opposite(), CompOp::LE);
        assert_eq!(CompOp::LE.as_opposite(), CompOp::GE);
        assert_eq!(CompOp::LT.as_opposite(), CompOp::GT);
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

    #[test]
    fn factor() {
        assert_eq!(CompOp::EQ.factor(), 0);
        assert_eq!(CompOp::NE.factor(), 0);
        assert_eq!(CompOp::LT.factor(), -1);
        assert_eq!(CompOp::LE.factor(), -1);
        assert_eq!(CompOp::GE.factor(), 1);
        assert_eq!(CompOp::GT.factor(), 1);
    }
}