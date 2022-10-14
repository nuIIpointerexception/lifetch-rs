use core::fmt::Display;

extern crate alloc;
use alloc::{format, string::String, vec::Vec};

#[derive(Debug, Clone)]
pub struct Builder<'a> {
    swap: bool,
    inner: Vec<Vec<&'a str>>,
    tabsize: usize,
    largest: usize,
}

impl<'a> From<Vec<Vec<&'a str>>> for Builder<'a> {
    /// ## Creates a new Builder with two vectors of strings, one for the art and one for the modules.
    /// The art is the first vector and the modules is the second vector.
    /// ### It can be swapped by setting the swap function to true.
    ///
    /// # Arguments:
    /// * `art`: The art vector.
    /// * `modules`: The modules vector.
    ///
    /// # Returns:
    /// The new Builder.
    ///
    /// # Example:
    /// ```rust
    /// use builder::Builder;
    ///
    /// let art = vec!["Hello", "World"];
    /// let modules = vec!["Rust", "is", "awesome"];
    /// let mut builder = Builder::from(vec![art, modules]);
    /// ```
    ///
    fn from(f: Vec<Vec<&'a str>>) -> Self {
        Self {
            swap: {
                let mut largest = 0;
                for line in f.iter() {
                    if line.len() > largest {
                        largest = line.len();
                    }
                }
                largest > 80
            },
            inner: f.clone(),
            tabsize: {
                let mut size = 0;
                for line in f.iter() {
                    for word in line.iter() {
                        size = size.max(word.len());
                    }
                }
                size + 3
            },
            largest: {
                let mut largest_line_count = 0;
                for walls in &f {
                    if largest_line_count < walls.len() {
                        largest_line_count = walls.len();
                    }
                }
                largest_line_count
            },
        }
    }
}

impl<'a> Display for Builder<'a> {
    /// This function returns the final Output of the Builder.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for line in &self.inner {
            for item in line {
                write!(f, "{:width$}", item, width = self.tabsize)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Builder<'a> {
    #[allow(dead_code)]
    /// ### Swaps the art and modules vectors.
    /// ### This function is only available if the swap function is set to true.
    /// ### It defaults to false.
    ///
    /// # Arguments:
    /// * `swap`: Should the art and modules be swapped?
    ///
    /// # Returns:
    /// The Builder with the art and modules swapped.
    ///
    /// # Example:
    /// ```rust
    /// use builder::Builder;
    ///
    /// let art = vec!["Hello", "World"];
    /// let modules = vec!["Rust", "is", "awesome"];
    /// let mut builder = Builder::from(vec![art, modules]);
    /// builder.swap(true);
    /// ```
    ///
    pub fn swap(&self, swap: bool) -> Self {
        if swap {
            let mut builder = self.clone();
            builder.inner.reverse();
            return builder;
        }
        self.clone()
    }

    #[must_use]
    /// ## Creates a new Fetch Builder with the given tabsize.
    /// ## The tabsize is the number of spaces that will be used to indent each line.
    /// ## The default tabsize is 15.
    /// # Arguments
    /// * `tabsize`: The number of spaces to use for indenting each line.
    /// # Returns
    /// The new Builder.
    /// # Example
    /// ```
    /// use builder::Builder;
    /// let mut builder = Builder::new(15);
    /// ```
    ///
    pub fn column_builder(&self) -> String {
        let mut i = 0;
        let mut f = String::new();
        while i < self.largest {
            let mut line = String::new();
            for item in &self.inner {
                let wall_item = item.get(i).unwrap_or(&"");
                line += wall_item;
                if wall_item.len() < self.tabsize
                    && self.inner.iter().position(|f| f == item) != Some(self.inner.len() - 1)
                {
                    line += &create_spaces(self.tabsize - wall_item.len());
                }
            }
            i += 1;
            f += &format!("{}\n", line);
        }
        f
    }

    /// Sets the tabsize for the builder.
    /// # Example
    /// ```
    /// use builder::Builder;
    /// let mut b = Builder::new();
    /// b.set_tabsize(15);
    /// ```
    ///
    pub fn set_tab(self, tabsize: usize) -> Self {
        Self {
            inner: self.inner,
            tabsize,
            largest: self.largest,
            swap: self.swap,
        }
    }
}

/// It takes a size and returns a String of spaces of that size
///
/// # Arguments
/// * `size`: The size of the String to return
///
/// # Returns
/// A String of spaces of the given size.
///
/// # Example
/// ```
/// use builder::spaces;
///
/// assert_eq!(spaces(5), "     ");
/// ```
///
pub fn create_spaces(size: usize) -> String {
    let mut spaces = String::new();
    let mut i = 0;
    while i < size {
        spaces += " ";
        i += 1;
    }
    spaces
}

#[cfg(test)]
pub(crate) mod builder_tests {
    use super::*;

    /// ## A macro for getting the test functions names.
    /// ## Source: https://stackoverflow.com/questions/38088067/
    /// # Example
    /// ```
    /// fn test_function() {
    ///     println!("{}", func!());
    /// }
    /// // Outputs "test_function".
    /// ```
    /// # Returns
    /// The name of the function as a str.
    ///
    macro_rules! func {
        () => {{
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);

            // Find and cut the rest of the path
            match &name[..name.len() - 3].rfind(':') {
                Some(pos) => &name[pos + 1..name.len() - 3],
                None => &name[..name.len() - 3],
            }
        }};
    }

    /// ## A macro for logging the tests in a more stylish way.
    /// Use it together with the func! macro to get a cool result.
    /// # Example
    /// ```
    /// #[test]
    /// fn test_function() {
    /// log!("test_function", "Output");
    /// }
    /// ```
    /// Output:
    /// ```console
    /// test_function ✔
    /// Output
    /// ```
    macro_rules! log {
        ($arg1:expr, $arg2:expr) => {
            let gray = "\x1b[38;5;242m";
            let reset = "\x1b[0m";
            let green = "\x1b[32m";
            println!(
                "{}{}{}{}{}{}{}",
                green, $arg1, " ✔\n", reset, gray, $arg2, reset
            );
        };
    }

    #[test]
    /// A quick test to make sure the builder works.
    /// # Expected Output
    /// ```
    /// a   d
    /// b   e
    /// c   f
    /// ```
    ///
    pub fn test_builder() {
        let b = Builder::from(vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
        let out = b.column_builder();
        assert_eq!(out, "a   d\nb   e\nc   f\n");
        log!(func!(), out);
    }

    #[test]
    /// It tests the tabsize of the builder.
    /// # Expected Output
    /// ```
    /// a          d
    /// b          e
    /// c          f
    /// ```
    ///
    pub fn test_tabsize() {
        let b = Builder::from(vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
        let out = b.set_tab(10).column_builder();
        assert_eq!(out, "a         d\nb         e\nc         f\n");
        log!(func!(), out);
    }

    #[test]
    /// It tests the tabsize of the builder, this time with a tabsize of 0.
    /// # Expected Output
    /// ```
    /// ad
    /// be
    /// cf
    /// ```
    ///
    pub fn test_tabsize_zero() {
        let b = Builder::from(vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
        let out = b.set_tab(0).column_builder();
        assert_eq!(out, "ad\nbe\ncf\n");
        log!(func!(), out);
    }

    #[test]
    /// Tries printing a practical example using the tabsize of 10.
    /// # Expected Output
    /// ```text
    /// Distro:   Arch Linux
    /// Packages: 42
    /// Size:     1.5GB
    /// Version:  2020.03.01
    /// ```
    pub fn test_practical() {
        let b = Builder::from(vec![
            vec!["Distro:", "Packages:", "Size:", "Version:"],
            vec!["Arch Linux", "42", "1.5GB", "2020.03.01"],
        ]);
        let out = b.set_tab(10).column_builder();
        assert_eq!(
            out,
            "Distro:   Arch Linux\nPackages: 42\nSize:     1.5GB\nVersion:  2020.03.01\n"
        );
        log!(func!(), out);
    }

    #[test]
    /// Tests the spaces function.
    /// # Expected Output
    /// ```text
    /// Have some spaces: [     ]
    /// ```
    ///
    pub fn test_spaces() {
        let out = create_spaces(5);
        assert_eq!(out, "     ");
        log!(func!(), out);
    }

    #[test]
    /// Test the swap feature.
    /// # Expected Output
    /// ```text
    /// d   a
    /// e   b
    /// f   c
    /// ```
    ///
    pub fn test_swap() {
        let b = Builder::from(vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
        let out = b.set_tab(10).swap(true).column_builder();
        assert_eq!(out, "d         a\ne         b\nf         c\n");
        log!(func!(), out);
    }
}
