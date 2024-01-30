use std::{ops::RangeInclusive, cmp::Ordering};

#[derive(Debug)]
pub struct FuzzySet {
    elements: Vec<(f32, f32)>,
    membership_interval: RangeInclusive<f32>,
}

impl FuzzySet {
    pub fn new(elements: Vec<(f32, f32)>, membership_interval: Option<RangeInclusive<f32>>) -> FuzzySet {
        match membership_interval {
            Some(membership_interval) => FuzzySet {elements, membership_interval},
            None => FuzzySet{elements, membership_interval: (0.0..=1.0)},
        }
    }

    pub fn add(&mut self, new_elem: (f32, f32)) {
        let (el, mem) = new_elem;
        
        if !self.membership_interval.contains(&mem) {
            panic!("{} is not in range {:?} of membership interval", mem, &self.membership_interval)
        }
        for (elem, _) in &self.elements {
            if *elem == el {
                return
            }
        }

        self.elements.push(new_elem)
    }

    pub fn remove() {

    }

    pub fn union() {

    }

    pub fn intersection() {

    }


}