//! Stable implementation of `Stub` using standard impls.

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

impl core::default::Default for Stub {
    fn default() -> Self {
        Self
    }
}

impl core::fmt::Debug for Stub {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str("Stub")
    }
}

impl core::fmt::Display for Stub {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str("{..}")
    }
}

impl core::clone::Clone for Stub {
    fn clone(&self) -> Self {
        *self
    }
}

impl core::marker::Copy for Stub {}
