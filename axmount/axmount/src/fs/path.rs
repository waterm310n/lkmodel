//! Path utility.

use alloc::str::Split;
use alloc::string::String;
use core::fmt::{Display, Formatter};

pub struct Path {
    inner: String,
}

impl Path {
    pub fn new(path: &str) -> Self {
        Self {
            inner: String::from(path),
        }
    }

    pub fn components(&self) -> Split<'_, char> {
        self.inner.split('/').into_iter()
    }

    pub fn parent(&self) -> Option<&str> {
        let mut iter = self.components();
        let _ = iter.next_back();
        iter.next_back().map(|p| if p == "" { "/" } else { p })
    }

    pub fn file_name(&self) -> Option<&str> {
        self.components().next_back()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
