//! Iterator module.
enum State {
    Default,
    Variable,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type<'t> {
    Default(&'t str),
    Variable(&'t str),
}

pub struct VariableIterator<'t> {
    text: &'t str,
    state: State,
    start: &'t str,
    end: &'t str,
}

impl<'t> VariableIterator<'t> {
    #[allow(dead_code)]
    pub fn new(text: &'t str, start: &'t str, end: &'t str) -> Self {
        Self {
            text,
            start,
            end,
            state: State::Default,
        }
    }

    fn parse_text(&mut self) -> Type<'t> {
        let token: Type;

        if let Some(placeholder_index) = self.text.find(self.start) {
            token = Type::Default(&self.text[..placeholder_index]);
            self.text = &self.text[placeholder_index..];
            self.state = State::Variable;
        } else {
            token = Type::Default(self.text);
            self.text = "";
        }

        token
    }

    fn parse_var(&mut self) -> Type<'t> {
        let token: Type;
        self.state = State::Default;

        if let Some(placeholder_index) = self.text.find(self.end) {
            token = Type::Variable(
                self.text[self.start.len()..placeholder_index]
                    .trim_start_matches(' ')
                    .trim_end_matches(' '),
            );
            let new_position = placeholder_index + self.end.len();
            self.text = &self.text[new_position..];
        } else {
            token = Type::Default(self.text);
            self.text = "";
        }

        token
    }
}

impl<'t> Iterator for VariableIterator<'t> {
    type Item = Type<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }

        match self.state {
            State::Default => Some(self.parse_text()),
            State::Variable => Some(self.parse_var()),
        }
    }
}
