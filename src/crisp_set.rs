use std::cmp::Ordering;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Set {
    elements: Vec<i32>,
}

impl  Set {
    pub fn new(elements: Vec<i32>) -> Set {
        Set{elements}
    }
    pub fn add(&mut self, value: i32) {
        for item in &self.elements {
            match value.cmp(item) {
                Ordering::Equal => return,
                _ => (),
            }
        }
        self.elements.push(value)
    }

    pub fn remove(&mut self, value: i32) {
        if let Some(index) = self.elements.iter().position(|&x| x == value) {
            self.elements.remove(index);
        }
    }

    pub fn union(&self, other: &Set) -> Set {
        let mut set = Set::new(Vec::new());

        for item in &self.elements {
            set.add(*item);
        }

        for item in &other.elements {
            set.add(*item);
        }
        set
    }

    pub fn intersection(&self, other: &Set) -> Set {
        let set = self.difference(other);
        let set = self.difference(&set);
        set
    }

    pub fn difference(&self, other: &Set) -> Set {
        let mut set = Set::new(Vec::new());
        for item in &self.elements {
            set.add(*item);
        }

        for item in &other.elements {
            set.remove(*item);
        }
        set
    }
}


//TODO: genericka mnozina pro ordering typy 
// fuzzy mnozina struct 
