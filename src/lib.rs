//! [MutStr](https://github.com/ThisAccountHasBeenSuspended/MutStr)
//! `MutStr` is a mutable alternative for `&str`.
//!
//! - `&str`
//! - `MutStr`
//! - - uses 16 bytes.
//! - `String`
//! - - uses 24 bytes.
//!
//! ### Example
//! ```
//! use mutstr::mutstr;
//! let mut result = mutstr::from("friend"); // Create
//! result += " :) :) :)"; // Add
//! result -= (2, " :)"); // Remove(2 times)
//! assert_eq!(result.as_str(), "friend :)");
//! ```

use std::{alloc, fmt, ops};

struct MutStrPtr(*mut u8, usize);
unsafe impl Send for MutStrPtr {}
unsafe impl Sync for MutStrPtr {}
impl MutStrPtr {
    #[inline(always)]
    fn raw(&self) -> *mut u8 {
        self.0
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.1
    }

    #[inline(always)]
    fn layout(&self) -> alloc::Layout {
        unsafe { alloc::Layout::from_size_align_unchecked(self.size(), 1) }
    }

    fn realloc(&mut self, new_size: usize) {
        unsafe {
            let old_layout = self.layout();
            self.0 = alloc::realloc(self.raw(), old_layout, new_size);
        };
        self.1 = new_size;
    }
}

#[cfg(feature = "drop")]
impl Drop for MutStrPtr {
    fn drop(&mut self) {
        if self.size() != 0 {
            unsafe {
                alloc::dealloc(self.raw(), self.layout());
            };
        }
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc");
/// assert_eq!(result.size(), 3);
/// ```
#[allow(non_camel_case_types)]
pub struct mutstr {
    #[doc(hidden)]
    _ptr: MutStrPtr,
}

impl mutstr {
    /// The raw pointer of the allocated heap.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let result = mutstr::from("abc");
    /// println!("{:?}", result.ptr());
    /// ```
    #[inline(always)]
    pub fn ptr(&self) -> *const u8 {
        self._ptr.raw() as *const u8
    }

    /// The raw pointer of the allocated heap.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("abc");
    /// unsafe {
    ///     println!("{:?}", result.ptr_mut());
    /// };
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn ptr_mut(&mut self) -> *mut u8 {
        self._ptr.raw()
    }

    /// The size of the data (is similar to the `len()` function of e.g. `&str`).
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("abc");
    /// assert_eq!(result.size(), 3);
    /// ```
    #[inline(always)]
    pub fn size(&self) -> usize {
        self._ptr.size()
    }

    /// Get the pointer layout.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let result = mutstr::from("abc");
    /// let result_layout = result.layout();
    /// assert_eq!(result_layout.size(), 3);
    /// ```
    #[inline(always)]
    pub fn layout(&self) -> alloc::Layout {
        self._ptr.layout()
    }

    /// Get the allocated data as `&[u8]`.
    ///
    /// **Notice:** _Can be used to compare with `&str`_.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let first = "abc";
    /// let second = mutstr::from("abc");
    /// assert_eq!(first.as_bytes(), second.as_bytes());
    /// ```
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.size()) }
    }

    /// Get the allocated data as `&mut [u8]`.
    ///
    /// **Notice:** _Like `as_bytes()` but mutable_.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("Hello");
    ///
    /// unsafe {
    ///     let mut bytes = result.as_bytes_mut();
    ///     bytes[0] = 0x6f; // o
    ///     bytes[1] = 0x6c; // l
    ///     bytes[2] = 0x6c; // l
    ///     bytes[3] = 0x65; // e
    ///     bytes[4] = 0x48; // H
    /// };
    ///
    /// assert_eq!(result.as_bytes(), b"olleH");
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.ptr_mut(), self.size())
    }

    /// Get the allocated data as `&str`.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let result = mutstr::from("abc");
    /// assert_eq!(result.as_str(), "abc");
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Get the allocated data as `&mut str`.
    ///
    /// **Notice:** _Like `as_str()` but mutable_
    #[inline(always)]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        std::str::from_utf8_unchecked_mut(self.as_bytes_mut())
    }

    /// Reallocates the existing heap if the size is not the same and write `new_value` into it.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("abc");
    /// result.replace_with("123");
    /// assert_eq!(result.as_str(), "123");
    /// ```
    pub fn replace_with(&mut self, new_value: &str) {
        let new_value_size = std::mem::size_of_val(new_value);
        if self.size() != new_value_size {
            self._ptr.realloc(new_value_size);
        }
        unsafe {
            std::ptr::copy(new_value.as_ptr(), self.ptr_mut(), new_value_size);
        };
    }

    /// Reallocates the existing heap and write `value` at the end of the data.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("abc");
    /// result.push("123");
    /// assert_eq!(result.as_str(), "abc123");
    /// ```
    pub fn push(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }

        let value_size = std::mem::size_of_val(value);
        let old_size = self.size();
        self._ptr.realloc(old_size + value_size);

        unsafe {
            let dst_ptr = self.ptr_mut().add(old_size);
            std::ptr::copy(value.as_ptr(), dst_ptr, value_size);
        };
    }

    /// Reallocates the existing heap to `0`, to free memory.
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("abc");
    /// assert_eq!(result.size(), 3);
    /// result.clear();
    /// assert_eq!(result.size(), 0);
    /// ```
    pub fn clear(&mut self) {
        self._ptr.realloc(0);
    }
}

impl From<&str> for mutstr {
    fn from(value: &str) -> Self {
        let value_size = std::mem::size_of_val(value);
        unsafe {
            let value_layout: alloc::Layout =
                alloc::Layout::from_size_align_unchecked(value_size, 1);
            let new_ptr: *mut u8 = alloc::alloc(value_layout);
            std::ptr::copy(value.as_ptr(), new_ptr, value_size);
            Self {
                _ptr: MutStrPtr(new_ptr, value_size),
            }
        }
    }
}

impl Default for mutstr {
    #[inline]
    fn default() -> Self {
        Self::from("")
    }
}

impl fmt::Display for mutstr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for mutstr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("mutstr").field("_ptr", &self.ptr()).finish()
    }
}

impl ops::Index<ops::RangeFull> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &str {
        self.as_str()
    }
}

impl ops::Index<ops::Range<usize>> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &str {
        &self[..][index]
    }
}

impl ops::Index<ops::RangeTo<usize>> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &str {
        &self[..][index]
    }
}

impl ops::Index<ops::RangeFrom<usize>> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &str {
        &self[..][index]
    }
}

impl ops::AddAssign<&str> for mutstr {
    #[inline]
    fn add_assign(&mut self, rhs: &str) {
        self.push(rhs);
    }
}

impl ops::SubAssign<&str> for mutstr {
    #[inline]
    fn sub_assign(&mut self, rhs: &str) {
        let new_value = self.as_str().replace(rhs, "");
        self.replace_with(&new_value);
    }
}

impl ops::SubAssign<(usize, &str)> for mutstr {
    #[inline]
    fn sub_assign(&mut self, rhs: (usize, &str)) {
        let new_value = self.as_str().replacen(rhs.1, "", rhs.0);
        self.replace_with(&new_value);
    }
}

#[cfg(feature = "serde")]
include!("serde.rs");

#[cfg(test)]
mod implementations {
    use super::mutstr;

    #[test]
    fn from() {
        let result = mutstr::from("abc123");
        assert_eq!(result.as_str(), "abc123");
    }

    #[test]
    fn default() {
        let result = mutstr::default();
        assert_eq!(result.as_str(), "");
    }

    #[test]
    fn display() {
        let result = format!("{:?}", mutstr::from("abc123"));
        if !result.starts_with("mutstr { _ptr: ") {
            assert_eq!('a', 'b');
        }
    }
    
    #[test]
    fn range_full() {
        let result = mutstr::from("abc123");
        assert_eq!(&result[..], "abc123");
    }

    #[test]
    fn range() {
        let result = mutstr::from("abc123");
        assert_eq!(&result[0..6], "abc123");
    }

    #[test]
    fn range_to() {
        let result = mutstr::from("abc123");
        assert_eq!(&result[..6], "abc123");
    }

    #[test]
    fn range_from() {
        let result = mutstr::from("abc123");
        assert_eq!(&result[0..], "abc123");
    }

    #[test]
    fn add_assign() {
        let mut result = mutstr::from("Hello my");
        result += " friend";
        assert_eq!(result.as_str(), "Hello my friend");
    }

    #[test]
    fn sub_assign() {
        let mut result = mutstr::from("Hello my friend");
        result -= " friend";
        assert_eq!(result.as_str(), "Hello my");
    }

    #[test]
    fn sub_assign_extended() {
        let mut result = mutstr::from("Hello my friend");
        result -= (1, " friend");
        assert_eq!(result.as_str(), "Hello my");
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_implementation {
    use super::mutstr;

    #[derive(Default, serde::Deserialize, serde::Serialize)]
    struct MyStruct {
        name: Option<mutstr>,
    }
    
    #[test]
    fn from_to() {
        let raw = r#"{"name":"Nick"}"#;
        let result = serde_json::from_str::<MyStruct>(raw).unwrap();
        assert_eq!(result.name.as_ref().unwrap().as_str(), "Nick");
        assert_eq!(serde_json::to_string(&result).unwrap(), raw);
    }
}
