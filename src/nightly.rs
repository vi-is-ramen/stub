//! Nightly implementation of `Stub` with const traits when possible.

/// A zero-sized type that implements common traits.
///
/// It can be used as a placeholder in generics (e.g., `T = Stub`) or as a
/// default value for functions that expect a type implementing certain traits.
///
/// # Examples
///
/// ```
/// use stub::Stub;
///
/// fn default_value<T: Default = Stub>() -> T {
///     T::default()
/// }
///
/// let s = default_value();
/// assert_eq!(format!("{}", s), "{..}");
/// ```
pub struct Stub;

#[cfg(not(feature = "no_core"))]
const impl core::default::Default for Stub {
    fn default() -> Self {
        Self
    }
}

#[cfg(not(feature = "no_core"))]
impl core::fmt::Debug for Stub {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str("Stub")
    }
}

#[cfg(not(feature = "no_core"))]
impl core::fmt::Display for Stub {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str("{..}")
    }
}

#[cfg(not(feature = "no_core"))]
const impl core::clone::Clone for Stub {
    fn clone(&self) -> Self {
        Self
    }
}
