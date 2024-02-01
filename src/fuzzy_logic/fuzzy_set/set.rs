use std:: ops::RangeInclusive;


#[derive(Debug)]
pub struct FuzzySet<T> {
    pub(crate) elements: Vec<(T, f32)>,
    membership_interval: RangeInclusive<f32>,
}

impl<T: PartialOrd + Copy> FuzzySet<T> {
    pub fn new(elements: Vec<(T, f32)>, membership_interval: Option<RangeInclusive<f32>>) -> FuzzySet<T> {
        match membership_interval {
            Some(membership_interval) => FuzzySet {elements, membership_interval},
            None => FuzzySet{elements, membership_interval: (0.0..=1.0)},
        }
    }

    pub fn add(&mut self, new_elem: (T, f32)) {
        let (elem, mem) = new_elem;
        
        if !self.is_membership_within_interval(mem) {
            panic!("{} is not in range {:?} of membership interval", mem, &self.membership_interval)
        }

        for (el, _) in &self.elements {
            if elem == *el {
                return
            }
        }

        self.elements.push(new_elem)
    }


    pub fn remove(&mut self, elem: T) {
        if let Some(index) = self.elements.iter().position(|(x, _)| *x == elem) {
            self.elements.remove(index);
        } else {
            panic!("element not in Fuzzy set")
        }
    }

    pub fn get_elem_membership(&self, elem: T) -> Option<f32> {
        for (el, mem) in &self.elements {
            if elem == *el {
                return Some(*mem)
            }
        }
        None
    }

    pub fn change_elem_membership(&mut self, elem: T, new_mem: f32) {
        for (el, mem) in self.elements.iter_mut() {
            if *el == elem {
                *mem = new_mem;
            }
        }
    }

    pub fn is_membership_within_interval(&self, mem: f32) -> bool {
        self.membership_interval.contains(&mem)
    }

    pub fn elements(&self) -> Vec<T> {
        let elements: Vec<T> = self.elements.iter().map(|&(elem,_)| elem).collect();
        elements
    }

    pub fn memberships(&self) -> Vec<f32> {
        let memberships: Vec<f32> = self.elements.iter().map(|&(_ ,mem)| mem).collect();
        memberships
    }

    pub fn element_with_membership(&self) -> &Vec<(T, f32)> {
        &self.elements
    }
}