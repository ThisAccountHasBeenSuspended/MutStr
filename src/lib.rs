//! [MutStr](https://github.com/ThisAccountHasBeenSuspended/MutStr)
//! is a good solution if you want to reduce memory consumption for e.g. hash tables like `<String, String>`
//! and need something more efficient than `Box<str>` because you have to change the data at runtime.
//!
//! [MutStr](https://github.com/ThisAccountHasBeenSuspended/MutStr)
//! uses 16 bytes.
//! [Box](https://github.com/rust-lang/rust/blob/master/library/alloc/src/boxed.rs)
//! uses 16 bytes.
//! [String](https://github.com/rust-lang/rust/blob/master/library/alloc/src/string.rs)
//! uses 24 bytes.
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

#[doc(hidden)]
struct MutStrPtr(*mut u8, usize); // size = 16 (0x10), align = 0x8
#[doc(hidden)]
unsafe impl Send for MutStrPtr {}
#[doc(hidden)]
unsafe impl Sync for MutStrPtr {}
#[doc(hidden)]
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

    fn realloc(&mut self, new_value_size: usize) {
        unsafe {
            let old_layout = self.layout();
            self.0 = alloc::realloc(self.raw(), old_layout, new_value_size);
        };
        self.1 = new_value_size;
    }
}

#[doc(hidden)]
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
    // size = 16 (0x10), align = 0x8
    #[doc(hidden)]
    _ptr: MutStrPtr,
}

impl mutstr {
    /// The raw pointer of the allocated heap
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

    /// The raw pointer of the allocated heap
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("abc");
    /// println!("{:?}", result.ptr_mut());
    /// ```
    #[inline(always)]
    pub fn ptr_mut(&mut self) -> *mut u8 {
        self._ptr.raw()
    }

    /// The size of the data (is similar to the `len()` function of e.g. `&str`)
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

    /// Get the pointer layout
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

    /// Get the allocated data as `&[u8]`
    ///
    /// **Notice:** _Can be used to compare with `&str` or `String`_
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let first = String::from("abc");
    /// let second = mutstr::from("abc");
    /// assert_eq!(first.as_bytes(), second.as_bytes());
    /// ```
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.size()) }
    }

    /// Get the allocated data as `&mut [u8]`
    ///
    /// **Notice:** _Like `as_bytes()` but mutable_
    ///
    /// ### Example
    /// ```
    /// use mutstr::mutstr;
    /// let mut result = mutstr::from("Hello");
    ///
    /// let mut bytes = result.as_bytes_mut();
    /// bytes[0] = 0x6f; // o
    /// bytes[1] = 0x6c; // l
    /// bytes[2] = 0x6c; // l
    /// bytes[3] = 0x65; // e
    /// bytes[4] = 0x48; // H
    ///
    /// assert_eq!(result.as_bytes(), b"olleH");
    /// ```
    #[inline(always)]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr_mut(), self.size()) }
    }

    /// Get the allocated data as `&str`
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

    /// Get the allocated data as `&mut str`
    ///
    /// **Notice:** _Like `as_str()` but mutable_
    #[inline(always)]
    pub fn as_str_mut(&mut self) -> &mut str {
        unsafe { std::str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }

    /// Reallocates the existing heap if the size is not the same and write `new_value` into it
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

    /// Reallocates the existing heap and write `value` at the end of the data
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

    /// Reallocates the existing heap to `0`, to free memory
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

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc123");
/// assert_eq!(result.as_str(), "abc123");
/// ```
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

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from(String::from("abc123"));
/// assert_eq!(result.as_str(), "abc123");
/// ```
impl From<String> for mutstr {
    #[inline]
    fn from(value: String) -> Self {
        Self::from(&*value)
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::default();
/// assert_eq!(result.as_str(), "");
/// ```
impl Default for mutstr {
    #[inline]
    fn default() -> Self {
        Self::from("")
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc123");
/// assert_eq!(result.to_string(), String::from("abc123"));
/// ```
impl ToString for mutstr {
    #[inline]
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc123");
/// //println!("{:?}", result); // mutstr { _ptr: ... }
/// ```
impl fmt::Debug for mutstr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("mutstr").field("_ptr", &self.ptr()).finish()
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc123");
/// assert_eq!(&result[..], "abc123");
/// ```
impl ops::Index<ops::RangeFull> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &str {
        self.as_str()
    }
}
/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc123");
/// assert_eq!(&result[0..6], "abc123");
/// ```
impl ops::Index<ops::Range<usize>> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &str {
        &self[..][index]
    }
}
/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc123");
/// assert_eq!(&result[..6], "abc123");
/// ```
impl ops::Index<ops::RangeTo<usize>> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &str {
        &self[..][index]
    }
}
/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("abc123");
/// assert_eq!(&result[0..], "abc123");
/// ```
impl ops::Index<ops::RangeFrom<usize>> for mutstr {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &str {
        &self[..][index]
    }
}

/// ```
/// use mutstr::mutstr;
/// let mut test = mutstr::from("Hello my");
/// test += " friend";
/// assert_eq!(test.as_str(), "Hello my friend");
/// ```
impl ops::AddAssign<&str> for mutstr {
    #[inline]
    fn add_assign(&mut self, rhs: &str) {
        self.push(rhs);
    }
}

/// ```
/// use mutstr::mutstr;
/// let mut test = mutstr::from("Hello my");
/// test += " friend".to_owned();
/// assert_eq!(test.as_str(), "Hello my friend");
/// ```
impl ops::AddAssign<String> for mutstr {
    #[inline]
    fn add_assign(&mut self, rhs: String) {
        self.push(&rhs);
    }
}

/// ```
/// use mutstr::mutstr;
/// let mut test = mutstr::from("Hello my friend");
/// test -= " friend";
/// assert_eq!(test.as_str(), "Hello my");
/// ```
impl ops::SubAssign<&str> for mutstr {
    #[inline]
    fn sub_assign(&mut self, rhs: &str) {
        let new_value = self.as_str().replace(rhs, "");
        self.replace_with(&new_value);
    }
}

/// ```
/// use mutstr::mutstr;
/// let mut test = mutstr::from("Hello my friend");
/// test -= (1, " friend");
/// assert_eq!(test.as_str(), "Hello my");
/// ```
impl ops::SubAssign<(usize, &str)> for mutstr {
    #[inline]
    fn sub_assign(&mut self, rhs: (usize, &str)) {
        let new_value = self.as_str().replacen(rhs.1, "", rhs.0);
        self.replace_with(&new_value);
    }
}

/// ```
/// use mutstr::mutstr;
/// let mut test = mutstr::from("Hello my friend");
/// test -= " friend".to_owned();
/// assert_eq!(test.as_str(), "Hello my");
/// ```
impl ops::SubAssign<String> for mutstr {
    #[inline]
    fn sub_assign(&mut self, rhs: String) {
        let new_value = self.as_str().replace(&*rhs, "");
        self.replace_with(&new_value);
    }
}

/// ```
/// use mutstr::mutstr;
/// let mut test = mutstr::from("Hello my friend");
/// test -= (1, " friend".to_owned());
/// assert_eq!(test.as_str(), "Hello my");
/// ```
impl ops::SubAssign<(usize, String)> for mutstr {
    #[inline]
    fn sub_assign(&mut self, rhs: (usize, String)) {
        let new_value = self.as_str().replacen(&*rhs.1, "", rhs.0);
        self.replace_with(&new_value);
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("test");
/// let result_str = &*result;
/// assert_eq!("test", result_str);
/// ```
impl ops::Deref for mutstr {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let mut result = mutstr::from("test");
/// let result_str = &mut *result;
/// // ...
/// ```
impl ops::DerefMut for mutstr {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_str_mut()
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let result = mutstr::from("test");
/// let result_str: &str = &result;
/// assert_eq!("test", result_str);
/// ```
impl AsRef<str> for mutstr {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

/// ### Example
/// ```
/// use mutstr::mutstr;
/// let mut result = mutstr::from("test");
/// let result_str: &mut str = &mut result;
/// // ...
/// ```
impl AsMut<str> for mutstr {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

#[cfg(feature = "serde")]
include!("serde.rs");
