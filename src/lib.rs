#[derive(Default, Debug, Clone)]
pub struct Rules<'a, T>(Vec<(&'a [T], &'a [T])>);

impl<'a, T> Rules<'a, T> {
    pub fn from_rules(rules: &[(&'a [T], &'a [T])]) -> Self {
        Rules(rules.to_owned())
    }

    pub fn with_rule(mut self, pred: &'a [T], succ: &'a [T]) -> Self {
        self.0.push((pred, succ));
        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct LSystem<'a, T: Clone + PartialEq> {
    rules: &'a Rules<'a, T>,
    string: Vec<T>,
}

impl<'a, T: Clone + PartialEq> LSystem<'a, T> {
    pub fn new(rules: &'a Rules<'a, T>, axiom: Vec<T>) -> Self {
        LSystem {
            rules,
            string: axiom,
        }
    }

    pub fn advance(&self) -> LSystem<'a, T> {
        let mut new_string: Vec<T> = Vec::with_capacity(self.string.len());

        let mut i = 0;
        while i < self.string.len() {
            let sub = &self.string[i..];

            if let Some((pred, succ)) =
                self.rules.0.iter().find(|(pred, _)| sub.starts_with(pred))
            {
                new_string.extend_from_slice(succ);
                i += pred.len();
            } else {
                new_string.push(sub[0].clone());
                i += 1;
            }
        }

        LSystem {
            rules: self.rules,
            string: new_string,
        }
    }
}

impl<'a, T: Clone + PartialEq> Iterator for LSystem<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.string.clone();
        *self = self.advance();
        Some(cur)
    }
}
