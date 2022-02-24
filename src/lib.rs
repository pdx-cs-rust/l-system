//! A basic implementation of
//! [L-systems](https://en.wikipedia.org/wiki/L-system) in Rust.
//!
//! ## Example
//!
//! The following is an implementation of [Lindenmayer's algae
//! L-system](https://en.wikipedia.org/wiki/L-system#Example_1:_Algae):
//!
//! ```
//! use lsystem::{LSystem, Rules};
//!
//! let rules = Rules::default()
//!     .with_rule(&['A'], &['A', 'B'])
//!     .with_rule(&['B'], &['A']);
//!
//! let mut system = LSystem::new(&rules, vec!['A']);
//!
//! assert_eq!("A".chars().collect::<Vec<_>>(), system.next().unwrap());
//! assert_eq!("AB".chars().collect::<Vec<_>>(), system.next().unwrap());
//! assert_eq!("ABA".chars().collect::<Vec<_>>(), system.next().unwrap());
//! assert_eq!("ABAAB".chars().collect::<Vec<_>>(), system.next().unwrap());
//! assert_eq!("ABAABABA".chars().collect::<Vec<_>>(), system.next().unwrap());
//! assert_eq!("ABAABABAABAAB".chars().collect::<Vec<_>>(), system.next().unwrap());
//! ```

/// A set of rules for an L-system.
///
/// These rules are not checked for duplicates
#[derive(Default, Debug, Clone)]
pub struct Rules<'a, T>(Vec<(&'a [T], &'a [T])>);

impl<'a, T> Rules<'a, T> {
    /// Add a rule defined by a predecessor and successor.
    pub fn with_rule(mut self, pred: &'a [T], succ: &'a [T]) -> Self {
        self.0.push((pred, succ));
        self
    }

    /// The number of rules in this ruleset.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether there are no rules in this ruleset.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// An L-system, defined by a ruleset (see [`Rules`]) and a current string
/// state.
///
/// The set of allowed symbols within the system can include all possible values
/// of `T`, depending on the initial string and ruleset. If you need to restrict
/// the domain of the alphabet, it is recommended you create a wrapper type with
/// some sort of validation mechanism.
pub struct LSystem<'a, T: Clone + PartialEq> {
    rules: &'a Rules<'a, T>,
    string: Vec<T>,
}

impl<'a, T: Clone + PartialEq> LSystem<'a, T> {
    /// Construct a new L-system from a ruleset and initial string.
    pub fn new(rules: &'a Rules<'a, T>, axiom: Vec<T>) -> Self {
        LSystem {
            rules,
            string: axiom,
        }
    }

    /// Advance the system a single step by applying the rules to the inner
    /// string, returning the state of the new system and leaving this one
    /// intact.
    ///
    /// The current rules of this system will be passed to the new one.
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

    /// Set the rules used by the system.
    ///
    /// This will not otherwise affect the current state, but may affect the
    /// states of subsequent advancements of the system.
    pub fn set_rules(&mut self, rules: &'a Rules<'a, T>) {
        self.rules = rules;
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
