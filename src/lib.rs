//! This library is primarily concerned with generating
//! strings that can be used by your favorite terminal
//! stream manipulation system to display a progress bar
//! #### Example
//! 
//! ```
//! extern crate termion;
//! extern crate progress_string
//! 
//! fn main() {
//!     let bar = progress_string::BarBuilder::new()
//!                                         .total(10000)
//!                                         .include_percent()
//!                                         .get_bar();
//!     println!("starting the progress");
//!     for i in 0..10000 {
//!         bar.replace(i);
//!         print!("{}{}", termion::cursor::Left(bar.get_last_width(), bar.to_string())
//!         std::thread::sleep(100);
//!     }
//!     println!("done with progress");
//! }
//! ```

/// Represents a progress bar, this can be used
/// to get your progress string
pub struct Bar {
    pub current_partial: usize,
    pub total: usize,
    width: usize,
    empty_char: char,
    full_char: char,
    include_percent: bool,
    previous_text_width: usize,
}

impl Bar {
    /// copy the underlying bar into a new instance
    /// primarily used for the BarBuilder implementation
    fn copy(self) -> Bar {
        Bar {
            ..self
        }
    }
}

/// Helper struct for building a
/// progress bar
/// 
/// #### Examples
/// ```
/// let bar = BarBuilder::new()
///                     .total(1000000)
///                     .width(20)
///                     .empty_char('0')
///                     .full_char('X')
///                     .include_percent()
///                     .get_bar();
/// ```
/// the above would look something like this
/// `[XXXXXXXXXX0000000000] 50.00%`
pub struct BarBuilder {
    bar: Bar,
}

impl BarBuilder {
    /// Create a new Bar Builder
    pub fn new() -> BarBuilder {
        BarBuilder {
            bar: Bar::default(),
        }
    }
    /// Add update the total (default 100)
    /// let thousand = BarBuilder::new().total(1000).get_bar();
    /// 
    /// #### Examples
    /// ```
    /// thousand.update(200);
    /// //yeilds [█                                                 ]
    /// ```
    pub fn total(mut self, total: usize) -> BarBuilder {
        self.bar.total = total;
        self
    }
    /// Update the progress section's width (default 50)
    /// ```
    /// let bar = BarBuilder::new().width(10);
    /// ///yeids [          ]
    /// ```
    pub fn width(mut self, width: usize) -> BarBuilder {
        self.bar.width = width;
        self
    }
    /// Update the character you want to use as an empty section
    /// of the progress bar (default ' ')
    /// 
    /// #### Examples
    /// ```
    /// let zero_emp = BarBuilder::new().empty_char('0').get_bar();
    /// yeilds
    /// // [██████████00000000000]
    /// ```
    pub fn empty_char(mut self, character: char) -> BarBuilder {
        self.bar.empty_char = character;
        self
    }
    /// Update the character you want to sue as a full section of
    /// the bar (default '█')
    /// 
    /// #### Examples
    /// ```
    /// let x_bar = BarBuilder::new().full_char('X').get_bar(); 
    /// //yeilds [XXXXXX      ]
    /// let y_bar = BarBuilder::new().full_char('Y').get_bar();
    /// //yeilds [YYYYYY      ]
    /// ```
    pub fn full_char(mut self, character: char) -> BarBuilder {
        self.bar.full_char = character;
        self
    }

    /// Update the bar to include the percent after the
    /// bar representation (default false)
    /// 
    /// #### Examples
    /// ```
    /// let no_p = BarBuilder::new().include_percent().get_bar();
    /// //yeilds [██████████          ] 50.00%
    /// let with_p = BarBuilder::new()
    /// //yeilds [██████████          ]
    /// ```
    pub fn include_percent(mut self) -> BarBuilder {
        self.bar.include_percent = true;
        self
    }
    /// Complete building your bar and return the
    /// updated struct
    /// 
    /// #### Examples
    /// ```
    /// let bar = BarBuilder::new().get_bar();
    /// //yeilds a default bar instance
    /// ```
    pub fn get_bar(self) -> Bar {
        self.bar.copy()
    }
}

impl Bar {
    ///Bar constructor with default values
    pub fn default() -> Bar {
        Bar {
            current_partial: 0,
            total: 100,
            width: 50,
            full_char:  '█',
            empty_char: ' ',
            include_percent: false,
            previous_text_width: 0
        }
    }
}

impl Bar {
    /// Update the current_partial value by adding the
    /// to_add parameter
    /// 
    /// #### Examples
    /// ```
    /// let mut bar = Bar::default();
    /// bar.update(10);
    /// assert_eq!(bar.current_partial, 110);
    /// ```
    pub fn update(&mut self, to_add: usize) {
        self.previous_text_width = self.get_width();
        self.current_partial += to_add;
    }
    /// Update the cureent partial by replacing the current
    /// value
    /// 
    /// #### Examples
    /// ```
    /// let mut bar = Bar::default();
    /// bar.replace(10);
    /// assert_eq!(bar.current_partial, 10);
    pub fn replace(&mut self, new_progress: usize) {
        self.previous_text_width = self.get_width();
        self.current_partial = new_progress;
    }
}

impl Bar {
    /// Get the string representation of the progress bar
    /// This stiring will include brackets ([]) around the
    /// empty/full characters, the width is determined by
    /// the width property.
    /// If `bar.include_percent == true`
    /// the resulting string will include a space and the 
    /// percent with 2 decimal palces followed by %.
    /// 
    /// #### Examples
    /// ```
    /// let mut with_percent = BarBuilder::new().include_percent()
    ///                                             .get_bar();
    /// with_percent.update(50);
    /// println!("{}", with_percent.to_string());
    /// //prints [█████████████████████████                         ] 50.00%
    /// let mut no_percent = BarBuilder::new().get_bar();
    /// no_percent.update(50);
    /// //prints [█████████████████████████                         ]
    /// ```
    pub fn to_string(&self) -> String {
        let percent = self.calculate_percent();
        let mut progress_bar = String::from("[");
        for i in 0..self.width {
            if (i as f32) < (self.width as f32 * percent) {
                progress_bar.push(self.full_char.clone());
            } else {
                progress_bar.push(self.empty_char.clone());
            }
        }
        progress_bar.push(']');
        if self.include_percent {
            progress_bar.push_str(format!(" {:.2}%", percent * 100.0).as_str());
        }
        progress_bar
    }
}

impl Bar {
    /// Get the current width of characters in the bar, this 
    /// includes the brackets, spaces and percent if set
    /// 
    /// #### Examples
    /// ```
    /// let bar = Bar::default();
    /// println!("{}", bar.get_width());
    /// //prints 52
    /// let mut with_percent = BarBuilder;;new().include_precent();
    /// println!("{}", bar.get_width());
    /// //prints 58
    /// with_percent.update(10);
    /// println!("{}", bar.get_width());
    /// //prints 59
    /// with_percent.replace(100);
    /// println!("{}", bar.get_width());
    /// //prints 60
    /// ```
    pub fn get_width(&self) -> usize {
        match self.include_percent {
            true => {
                let current_percent = self.calculate_percent();
                if current_percent >= 0.95 {
                    self.width + 10
                } else if current_percent > 0.095 {
                    self.width + 9
                } else {
                    self.width + 8
                }
            }
            _ => self.width + 2
        }
    }
    /// Similar to `get_width` but gets the value
    /// before the last `update` or `replace` call
    /// this is useful for when you are trying to clear
    /// the terminal
    pub fn get_last_width(&self) -> usize {
        self.previous_text_width
    }
}

impl Bar {
    fn calculate_percent(&self) -> f32 {
        (self.current_partial as f32 / self.total as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn update_test() {

    }

    #[test]
    fn replace_test() {

    }

    #[test]
    fn to_string_test() {
        
    }
}
