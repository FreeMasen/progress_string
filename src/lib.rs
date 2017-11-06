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

    pub fn copy(self) -> Bar {
        Bar {
            ..self
        }
    }
}

pub struct BarBuilder {
    bar: Bar,
}

impl BarBuilder {

    pub fn new() -> BarBuilder {
        BarBuilder {
            bar: Bar::default(),
        }
    }
    pub fn total(mut self, total: usize) -> BarBuilder {
        self.bar.total = total;
        self
    }

    pub fn width(mut self, width: usize) -> BarBuilder {
        self.bar.width = width;
        self
    }

    pub fn empty_char(mut self, character: char) -> BarBuilder {
        self.bar.empty_char = character;
        self
    }
    
    pub fn full_char(mut self, character: char) -> BarBuilder {
        self.bar.full_char = character;
        self
    }

    pub fn include_percent(mut self) -> BarBuilder {
        self.bar.include_percent = true;
        self
    }

    pub fn get_bar(self) -> Bar {
        self.bar.copy()
    }
}

impl Bar {

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

    pub fn update(&mut self, to_add: usize) {
        self.previous_text_width = self.get_width();
        self.current_partial += to_add;
    }

    pub fn replace(&mut self, new_progress: usize) {
        self.previous_text_width = self.get_width();
        self.current_partial = new_progress;
    }
}

impl Bar {

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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
