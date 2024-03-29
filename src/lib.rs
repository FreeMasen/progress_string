//! This library is primarily concerned with generating strings that can be used by your favorite
//! terminal stream manipulation system to display a progress bar.
//!
//! #### Example
//!
//! ```
//! # #[cfg(unix)]
//! use std::thread::sleep;
//! # #[cfg(unix)]
//! use std::time::Duration;
//!
//! # #[cfg(unix)]
//! const TOTAL: usize = 1000;
//! # #[cfg(unix)]
//! fn main() {
//!     let mut bar = progress_string::BarBuilder::new()
//!         .total(TOTAL)
//!         .include_percent()
//!         .build();
//!
//!     println!("starting the progress");
//!     for i in 0..TOTAL {
//!         bar.replace(i);
//!         print!(
//!             "{}{}",
//!             termion::cursor::Left(bar.get_last_width() as u16),
//!             bar.to_string()
//!         );
//!         sleep(Duration::from_millis(10));
//!     }
//!     println!("\ndone with progress");
//! }
//!
//! # #[cfg(not(unix))]
//! # fn main() {}
//!```

/// Represents a progress bar which can be used to get your progress string.
pub struct Bar {
    pub current_partial: usize,
    pub total: usize,
    width: usize,
    empty_char: char,
    full_char: char,
    leading_char: char,
    include_percent: bool,
    include_numbers: bool,
    previous_text_width: usize,
}

/// Helper struct for building a progress bar.
///
/// #### Examples
/// ```
/// use progress_string::BarBuilder;
///
/// let bar = BarBuilder::new()
///                     .total(1000000)
///                     .width(20)
///                     .empty_char('0')
///                     .full_char('X')
///                     .include_percent()
///                     .build();
/// ```
/// the above would look something like this
/// `[XXXXXXXXXX0000000000] 50.00%`
pub struct BarBuilder {
    bar: Bar,
}

impl Default for BarBuilder {
    fn default() -> Self {
        BarBuilder::new()
    }
}

impl BarBuilder {
    /// Create a new `BarBuilder`.
    pub fn new() -> Self {
        BarBuilder {
            bar: Bar::default(),
        }
    }
    /// Update the total (default 100).
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let thousand = BarBuilder::new().total(1000).build();
    /// // yields [█                                                 ]
    /// ```
    pub fn total(mut self, total: usize) -> BarBuilder {
        self.bar.total = total;
        self
    }
    /// Update the progress section's width (default 50).
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let bar = BarBuilder::new().width(10);
    /// // yields [          ]
    /// ```
    pub fn width(mut self, width: usize) -> BarBuilder {
        self.bar.width = width;
        self
    }
    /// Update the character you want to use as an empty section of the progress bar (default ' ').
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let zero_emp = BarBuilder::new().empty_char('0').build();
    /// // yields
    /// // [██████████00000000000]
    /// ```
    pub fn empty_char(mut self, character: char) -> BarBuilder {
        self.bar.empty_char = character;
        self
    }
    /// Update the character you want to use as a full section of the bar (default '█').
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let x_bar = BarBuilder::new().full_char('X').build();
    /// // yields [XXXXXX      ]
    /// let y_bar = BarBuilder::new().full_char('Y').build();
    /// // yields [YYYYYY      ]
    /// ```
    pub fn full_char(mut self, character: char) -> BarBuilder {
        self.bar.full_char = character;
        self
    }
    /// Update the character you want to use to lead the full section of the bar
    /// (defaults to the value of `full_char` if not provided).
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let x_bar = BarBuilder::new()
    ///                 .full_char('X')
    ///                 .leading_char('}')
    ///                 .build();
    /// // yields [XXXXXX}     ]
    /// let y_bar = BarBuilder::new()
    ///                 .full_char('Y')
    ///                 .leading_char(')')
    ///                 .build();
    /// // yields [YYYYYY)     ]
    /// ```
    pub fn leading_char(mut self, character: impl Into<Option<char>>) -> BarBuilder {
        if let Some(char) = character.into() {
            self.bar.leading_char = char;
        } else {
            self.bar.leading_char = self.bar.full_char;
        }
        self
    }

    /// Update the bar to include the percent after the bar representation (default `false`).
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let no_p = BarBuilder::new().include_percent().build();
    /// // yields [██████████          ] 50.00%
    /// let with_p = BarBuilder::new();
    /// // yields [██████████          ]
    /// ```
    pub fn include_percent(mut self) -> BarBuilder {
        self.bar.include_percent = true;
        self
    }
    /// Update the bar to include the divison after the bar representation.
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let mut no_n = BarBuilder::new().build();
    /// no_n.replace(50);
    /// // yields [██████████          ]
    /// let mut with_n = BarBuilder::new().include_numbers().build();
    /// with_n.replace(50)
    /// // yields [██████████          ] 50/100
    /// ```
    pub fn include_numbers(mut self) -> BarBuilder {
        self.bar.include_numbers = true;
        self
    }
    /// deprecated please use `build`
    #[deprecated]
    pub fn get_bar(self) -> Bar {
        self.bar
    }

    /// Complete building your bar and return the updated struct.
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let bar = BarBuilder::new().build();
    /// // yields a default bar instance
    /// ```
    pub fn build(self) -> Bar {
        self.bar
    }
}

impl Default for Bar {
    /// Bar constructor with default values.
    /// ```text
    /// Bar {
    ///     current_partial: 0,
    ///     total: 100,
    ///     width: 50,
    ///     full_char:  '█',
    ///     empty_char: ' ',
    ///     leading_char: '█',
    ///     include_percent: false,
    ///     include_numbers: false,
    ///     previous_text_width: 0
    /// }
    /// ```
    fn default() -> Self {
        Self {
            current_partial: 0,
            total: 100,
            width: 50,
            full_char: '█',
            empty_char: ' ',
            leading_char: '█',
            include_percent: false,
            include_numbers: false,
            previous_text_width: 0,
        }
    }
}

impl Bar {
    /// Update the `current_partial` value by adding the `to_add` parameter.
    ///
    /// #### Examples
    /// ```
    /// use progress_string::Bar;
    ///
    /// let mut bar = Bar::default();
    /// bar.update(10);
    /// assert_eq!(bar.current_partial, 10);
    /// ```
    pub fn update(&mut self, to_add: usize) {
        self.previous_text_width = self.get_width();
        self.current_partial += to_add;
    }
    /// Update the current partial by replacing the current value.
    ///
    /// #### Examples
    /// ```
    /// use progress_string::Bar;
    ///
    /// let mut bar = Bar::default();
    /// bar.replace(10);
    /// assert_eq!(bar.current_partial, 10);
    /// ```
    pub fn replace(&mut self, new_progress: usize) {
        self.previous_text_width = self.get_width();
        self.current_partial = new_progress;
    }
    /// Get the current width of characters in the bar.
    ///
    /// This includes the brackets, spaces and percent if set.
    ///
    /// #### Examples
    /// ```
    /// use progress_string::{Bar, BarBuilder};
    ///
    /// let bar = Bar::default();
    /// assert_eq!(bar.get_width(), 52);
    ///
    /// let mut with_percent = BarBuilder::new().include_percent().build();
    /// assert_eq!(with_percent.get_width(), 58);
    ///
    /// with_percent.update(10);
    /// assert_eq!(with_percent.get_width(), 59);
    ///
    /// with_percent.replace(100);
    /// assert_eq!(with_percent.get_width(), 60);
    /// ```
    pub fn get_width(&self) -> usize {
        let mut width: usize = 52;
        if self.include_numbers {
            let total_string = format!("{}", self.total);
            let partial_string = format!("{}", self.current_partial);
            width += total_string.len() + partial_string.len() + 2;
        }
        if self.include_percent {
            let current_percent = self.calculate_percent();
            if current_percent >= 0.95 {
                width += 8;
            } else if current_percent > 0.095 {
                width += 7;
            } else {
                width += 6;
            }
        }
        width
    }
    /// Similar to `get_width` but gets the value before the last `update` or `replace` call.
    ///
    /// This is useful for when you are trying to clear the terminal.
    pub fn get_last_width(&self) -> usize {
        self.previous_text_width
    }

    fn calculate_percent(&self) -> f32 {
        self.current_partial as f32 / self.total as f32
    }
}

impl std::fmt::Display for Bar {
    /// Get the string representation of the progress bar.
    ///
    /// This string will include brackets ([]) around the empty/full characters. The width is
    /// determined by the width property. If `bar.include_percent == true`, the resulting string
    /// will include a space and the percent with 2 decimal places followed by %.
    ///
    /// #### Examples
    /// ```
    /// use progress_string::BarBuilder;
    ///
    /// let mut with_percent = BarBuilder::new().include_percent().build();
    /// with_percent.update(50);
    /// println!("{}", with_percent.to_string());
    /// // prints [█████████████████████████                         ] 50.00%
    /// let mut no_percent = BarBuilder::new().build();
    /// no_percent.update(50);
    /// // prints [█████████████████████████                         ]
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let percent = self.calculate_percent();
        f.write_str("[")?;
        for i in 0..self.width {
            if (i as f32) < ((self.width as f32 * percent) - 1.0) {
                f.write_fmt(format_args!("{}", self.full_char))?;
            } else if (i as f32) < (self.width as f32 * percent) {
                f.write_fmt(format_args!("{}", self.leading_char))?;
            } else {
                f.write_fmt(format_args!("{}", self.empty_char))?;
            }
        }
        f.write_str("]")?;
        if self.include_percent {
            f.write_fmt(format_args!(" {:.2}%", percent * 100.0))?;
        }
        if self.include_numbers {
            f.write_fmt(format_args!(" {:?}/{:?}", self.current_partial, self.total))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include_percent_test() {
        let mut bar = BarBuilder::new().include_percent().build();
        // single digit percent
        assert_eq!(bar.get_width(), 58);
        assert_eq!(
            format!("{}", bar),
            "[                                                  ] 0.00%"
        );
        bar.update(50);
        // double digit percent
        assert_eq!(bar.get_width(), 59);
        assert_eq!(
            format!("{}", bar),
            "[█████████████████████████                         ] 50.00%"
        );
        bar.update(50);
        // triple digit percent
        assert_eq!(bar.get_width(), 60);
        assert_eq!(
            format!("{}", bar),
            "[██████████████████████████████████████████████████] 100.00%"
        );
    }

    #[test]
    fn include_numbers_test() {
        let mut bar = BarBuilder::new().include_numbers().build();
        // 0/100
        assert_eq!(bar.get_width(), 58);
        assert_eq!(
            format!("{}", bar),
            "[                                                  ] 0/100"
        );
        bar.update(50);
        // 50/100
        assert_eq!(bar.get_width(), 59);
        assert_eq!(
            format!("{}", bar),
            "[█████████████████████████                         ] 50/100"
        );
        bar.update(50);
        // 100/100
        assert_eq!(bar.get_width(), 60);
        assert_eq!(
            format!("{}", bar),
            "[██████████████████████████████████████████████████] 100/100"
        );
    }

    #[test]
    fn update_test() {
        let mut bar = Bar::default();
        bar.update(50);
        assert_eq!(bar.current_partial, 50);
        assert_eq!(
            format!("{}", bar),
            "[█████████████████████████                         ]"
        );
    }

    #[test]
    fn replace_test() {
        let mut bar = Bar::default();
        bar.update(50);
        assert_eq!(bar.current_partial, 50);
        assert_eq!(
            format!("{}", bar),
            "[█████████████████████████                         ]"
        );
        bar.replace(10);
        assert_eq!(bar.current_partial, 10);
        assert_eq!(
            format!("{}", bar),
            "[█████                                             ]"
        );
    }

    #[test]
    fn to_string_test() {
        let mut bar = Bar::default();
        assert_eq!(
            bar.to_string(),
            "[                                                  ]"
        );
        bar.update(50);
        assert_eq!(
            bar.to_string(),
            "[█████████████████████████                         ]"
        )
    }
    #[test]
    fn leading_char() {
        let mut bar = BarBuilder::new().leading_char('>').build();
        assert_eq!(
            bar.to_string(),
            "[                                                  ]"
        );
        bar.update(50);
        assert_eq!(
            bar.to_string(),
            "[████████████████████████>                         ]"
        )
    }
    #[test]
    fn display() {
        let mut bar = BarBuilder::new().build();
        assert_eq!(
            format!("{}", bar),
            "[                                                  ]"
        );
        bar.update(50);
        assert_eq!(
            format!("{}", bar),
            "[█████████████████████████                         ]"
        )
    }
}
