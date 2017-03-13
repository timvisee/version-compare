/// Comparison operators.
#[derive(Debug, Clone, PartialEq)]
pub enum CompOp {
    /// Equal to. (==)
    Eq,

    /// Not equal to. (!=)
    Ne,

    /// Less than. (<)
    Lt,

    /// Less than or equal to. (<=)
    Le,

    /// Greater than or equal to. (>=)
    Ge,

    /// Greater than. (>)
    Gt
}

impl CompOp {

    /// Get a comparison operator by it's sign.
    /// Whitespaces are stripped from the sign string.
    /// An error is returned if the sign isn't recognized.
    ///
    /// The following signs are supported:
    /// - ==: `Eq`
    /// - !=: `Ne`
    /// - <:  `Lt`
    /// - <=: `Le`
    /// - >=: `Ge`
    /// - >:  `Gt`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::from_sign("=="), Ok(CompOp::Eq));
    /// assert_eq!(CompOp::from_sign("<"), Ok(CompOp::Lt));
    /// assert_eq!(CompOp::from_sign("  >=   "), Ok(CompOp::Ge));
    /// assert!(CompOp::from_sign("*").is_err());
    /// ```
    pub fn from_sign(sign: &str) -> Result<CompOp, ()> {
        match sign.trim().as_ref() {
            "==" => Ok(CompOp::Eq),
            "!=" => Ok(CompOp::Ne),
            "<" => Ok(CompOp::Lt),
            "<=" => Ok(CompOp::Le),
            ">=" => Ok(CompOp::Ge),
            ">" => Ok(CompOp::Gt),
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
    /// assert_eq!(CompOp::from_name("eq"), Ok(CompOp::Eq));
    /// assert_eq!(CompOp::from_name("lt"), Ok(CompOp::Lt));
    /// assert_eq!(CompOp::from_name("  Ge   "), Ok(CompOp::Ge));
    /// assert!(CompOp::from_name("abc").is_err());
    /// ```
    pub fn from_name(sign: &str) -> Result<CompOp, ()> {
        match sign.trim().to_lowercase().as_ref() {
            "eq" => Ok(CompOp::Eq),
            "ne" => Ok(CompOp::Ne),
            "lt" => Ok(CompOp::Lt),
            "le" => Ok(CompOp::Le),
            "ge" => Ok(CompOp::Ge),
            "gt" => Ok(CompOp::Gt),
            _ => Err(())
        }
    }

    /// Get the name of this comparison operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.name(), "eq");
    /// assert_eq!(CompOp::Lt.name(), "lt");
    /// assert_eq!(CompOp::Ge.name(), "ge");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            &CompOp::Eq => "eq",
            &CompOp::Ne => "ne",
            &CompOp::Lt => "lt",
            &CompOp::Le => "le",
            &CompOp::Ge => "ge",
            &CompOp::Gt => "gt"
        }
    }

    /// Covert to the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - Eq <-> Ne
    /// - Lt <-> Ge
    /// - Le <-> Gt
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.as_inverted(), CompOp::Ne);
    /// assert_eq!(CompOp::Lt.as_inverted(), CompOp::Ge);
    /// assert_eq!(CompOp::Gt.as_inverted(), CompOp::Le);
    /// ```
    pub fn as_inverted(self) -> Self {
        self.invert()
    }

    /// Get the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - Eq <-> Ne
    /// - Lt <-> Ge
    /// - Le <-> Gt
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.invert(), CompOp::Ne);
    /// assert_eq!(CompOp::Lt.invert(), CompOp::Ge);
    /// assert_eq!(CompOp::Gt.invert(), CompOp::Le);
    /// ```
    pub fn invert(&self) -> Self {
        match self {
            &CompOp::Eq => CompOp::Ne,
            &CompOp::Ne => CompOp::Eq,
            &CompOp::Lt => CompOp::Ge,
            &CompOp::Le => CompOp::Gt,
            &CompOp::Ge => CompOp::Lt,
            &CompOp::Gt => CompOp::Le
        }
    }

    /// Convert to the opposite comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - Eq <-> Ne
    /// - Lt <-> Gt
    /// - Le <-> Ge
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.as_opposite(), CompOp::Ne);
    /// assert_eq!(CompOp::Lt.as_opposite(), CompOp::Gt);
    /// assert_eq!(CompOp::Ge.as_opposite(), CompOp::Le);
    /// ```
    pub fn as_opposite(self) -> Self {
        self.opposite()
    }

    /// Get the opposite comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - Eq <-> Ne
    /// - Lt <-> Gt
    /// - Le <-> Ge
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.opposite(), CompOp::Ne);
    /// assert_eq!(CompOp::Lt.opposite(), CompOp::Gt);
    /// assert_eq!(CompOp::Ge.opposite(), CompOp::Le);
    /// ```
    pub fn opposite(&self) -> Self {
        match self {
            &CompOp::Eq => CompOp::Ne,
            &CompOp::Ne => CompOp::Eq,
            &CompOp::Lt => CompOp::Gt,
            &CompOp::Le => CompOp::Ge,
            &CompOp::Ge => CompOp::Le,
            &CompOp::Gt => CompOp::Lt
        }
    }

    /// Convert to the flipped comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - Lt <-> Gt
    /// - Le <-> Ge
    /// - Other operators are returned as is.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.as_flipped(), CompOp::Eq);
    /// assert_eq!(CompOp::Lt.as_flipped(), CompOp::Gt);
    /// assert_eq!(CompOp::Ge.as_flipped(), CompOp::Le);
    /// ```
    pub fn as_flipped(self) -> Self {
        self.flip()
    }

    /// Get the flipped comparison operator.
    ///
    /// This uses the following bidirectional rules:
    /// - Lt <-> Gt
    /// - Le <-> Ge
    /// - Other operators are returned as is.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.flip(), CompOp::Eq);
    /// assert_eq!(CompOp::Lt.flip(), CompOp::Gt);
    /// assert_eq!(CompOp::Ge.flip(), CompOp::Le);
    /// ```
    pub fn flip(&self) -> Self {
        match self {
            &CompOp::Lt => CompOp::Gt,
            &CompOp::Le => CompOp::Ge,
            &CompOp::Ge => CompOp::Le,
            &CompOp::Gt => CompOp::Lt,
            _ => self.clone()
        }
    }

    /// Get the sign for this comparison operator.
    ///
    /// The following signs are returned:
    /// - Eq: `==`
    /// - Ne: `!=`
    /// - Lt: `<`
    /// - Le: `<=`
    /// - Ge: `>=`
    /// - Gt: `>`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    ///
    /// assert_eq!(CompOp::Eq.sign(), "==");
    /// assert_eq!(CompOp::Lt.sign(), "<");
    /// assert_eq!(CompOp::Ge.flip().sign(), "<=");
    /// ```
    pub fn sign(&self) -> &'static str {
        match self {
            &CompOp::Eq => "==",
            &CompOp::Ne => "!=",
            &CompOp::Lt => "<",
            &CompOp::Le => "<=",
            &CompOp::Ge => ">=",
            &CompOp::Gt => ">"
        }
    }

    /// Get a factor (number) for this comparison operator.
    /// These factors can be useful for quick calculations.
    ///
    /// The following factor numbers are returned:
    /// - Eq | Ne: `0`
    /// - Lt | Le: `-1`
    /// - Gt | Ge: `1`
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
            &CompOp::Eq | &CompOp::Ne => 0,
            &CompOp::Lt | &CompOp::Le => -1,
            &CompOp::Gt | &CompOp::Ge => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use comp_op::CompOp;

    #[test]
    fn from_sign() {
        // Normal signs
        assert_eq!(CompOp::from_sign("==").unwrap(), CompOp::Eq);
        assert_eq!(CompOp::from_sign("!=").unwrap(), CompOp::Ne);
        assert_eq!(CompOp::from_sign("<").unwrap(), CompOp::Lt);
        assert_eq!(CompOp::from_sign("<=").unwrap(), CompOp::Le);
        assert_eq!(CompOp::from_sign(">=").unwrap(), CompOp::Ge);
        assert_eq!(CompOp::from_sign(">").unwrap(), CompOp::Gt);

        // Exceptional cases
        assert_eq!(CompOp::from_sign("  <=  ").unwrap(), CompOp::Le);
        assert!(CompOp::from_sign("*").is_err());
    }

    #[test]
    fn from_name() {
        // Normal names
        assert_eq!(CompOp::from_name("eq").unwrap(), CompOp::Eq);
        assert_eq!(CompOp::from_name("ne").unwrap(), CompOp::Ne);
        assert_eq!(CompOp::from_name("lt").unwrap(), CompOp::Lt);
        assert_eq!(CompOp::from_name("le").unwrap(), CompOp::Le);
        assert_eq!(CompOp::from_name("ge").unwrap(), CompOp::Ge);
        assert_eq!(CompOp::from_name("gt").unwrap(), CompOp::Gt);

        // Exceptional cases
        assert_eq!(CompOp::from_name("  Le  ").unwrap(), CompOp::Le);
        assert!(CompOp::from_name("abc").is_err());
    }

    #[test]
    fn name() {
        assert_eq!(CompOp::Eq.name(), "eq");
        assert_eq!(CompOp::Ne.name(), "ne");
        assert_eq!(CompOp::Lt.name(), "lt");
        assert_eq!(CompOp::Le.name(), "le");
        assert_eq!(CompOp::Ge.name(), "ge");
        assert_eq!(CompOp::Gt.name(), "gt");
    }

    #[test]
    fn as_inverted() {
        assert_eq!(CompOp::Ne.as_inverted(), CompOp::Eq);
        assert_eq!(CompOp::Eq.as_inverted(), CompOp::Ne);
        assert_eq!(CompOp::Ge.as_inverted(), CompOp::Lt);
        assert_eq!(CompOp::Gt.as_inverted(), CompOp::Le);
        assert_eq!(CompOp::Lt.as_inverted(), CompOp::Ge);
        assert_eq!(CompOp::Le.as_inverted(), CompOp::Gt);
    }

    #[test]
    fn invert() {
        assert_eq!(CompOp::Ne.invert(), CompOp::Eq);
        assert_eq!(CompOp::Eq.invert(), CompOp::Ne);
        assert_eq!(CompOp::Ge.invert(), CompOp::Lt);
        assert_eq!(CompOp::Gt.invert(), CompOp::Le);
        assert_eq!(CompOp::Lt.invert(), CompOp::Ge);
        assert_eq!(CompOp::Le.invert(), CompOp::Gt);
    }

    #[test]
    fn as_opposite() {
        assert_eq!(CompOp::Ne.as_opposite(), CompOp::Eq);
        assert_eq!(CompOp::Eq.as_opposite(), CompOp::Ne);
        assert_eq!(CompOp::Gt.as_opposite(), CompOp::Lt);
        assert_eq!(CompOp::Ge.as_opposite(), CompOp::Le);
        assert_eq!(CompOp::Le.as_opposite(), CompOp::Ge);
        assert_eq!(CompOp::Lt.as_opposite(), CompOp::Gt);
    }

    #[test]
    fn opposite() {
        assert_eq!(CompOp::Eq.opposite(), CompOp::Ne);
        assert_eq!(CompOp::Ne.opposite(), CompOp::Eq);
        assert_eq!(CompOp::Lt.opposite(), CompOp::Gt);
        assert_eq!(CompOp::Le.opposite(), CompOp::Ge);
        assert_eq!(CompOp::Ge.opposite(), CompOp::Le);
        assert_eq!(CompOp::Gt.opposite(), CompOp::Lt);
    }

    #[test]
    fn as_flipped() {
        assert_eq!(CompOp::Eq.as_flipped(), CompOp::Eq);
        assert_eq!(CompOp::Ne.as_flipped(), CompOp::Ne);
        assert_eq!(CompOp::Lt.as_flipped(), CompOp::Gt);
        assert_eq!(CompOp::Le.as_flipped(), CompOp::Ge);
        assert_eq!(CompOp::Ge.as_flipped(), CompOp::Le);
        assert_eq!(CompOp::Gt.as_flipped(), CompOp::Lt);
    }

    #[test]
    fn flip() {
        assert_eq!(CompOp::Eq.flip(), CompOp::Eq);
        assert_eq!(CompOp::Ne.flip(), CompOp::Ne);
        assert_eq!(CompOp::Lt.flip(), CompOp::Gt);
        assert_eq!(CompOp::Le.flip(), CompOp::Ge);
        assert_eq!(CompOp::Ge.flip(), CompOp::Le);
        assert_eq!(CompOp::Gt.flip(), CompOp::Lt);
    }

    #[test]
    fn sign() {
        assert_eq!(CompOp::Eq.sign(), "==");
        assert_eq!(CompOp::Ne.sign(), "!=");
        assert_eq!(CompOp::Lt.sign(), "<");
        assert_eq!(CompOp::Le.sign(), "<=");
        assert_eq!(CompOp::Ge.sign(), ">=");
        assert_eq!(CompOp::Gt.sign(), ">");
    }

    #[test]
    fn factor() {
        assert_eq!(CompOp::Eq.factor(), 0);
        assert_eq!(CompOp::Ne.factor(), 0);
        assert_eq!(CompOp::Lt.factor(), -1);
        assert_eq!(CompOp::Le.factor(), -1);
        assert_eq!(CompOp::Ge.factor(), 1);
        assert_eq!(CompOp::Gt.factor(), 1);
    }
}