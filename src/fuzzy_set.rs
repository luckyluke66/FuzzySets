use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct FuzzySet<T> {
    elements: Vec<(T, f32)>,
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

    pub fn union(&self, other: &FuzzySet<T>) -> FuzzySet<T> {
        self.zadeh_operation(other, 
            |x: f32, y: f32| x.max(y))
    }

    pub fn intersection(&self, other: &FuzzySet<T>) -> FuzzySet<T> {
        self.zadeh_operation(other, 
            |x: f32, y: f32| x.min(y))
    }

    pub fn implication() {

    }

    pub fn defuzzyfication() {

    }
}

impl<T: PartialOrd + Copy> FuzzySet<T> {
    fn is_membership_within_interval(&self, mem: f32) -> bool {
        self.membership_interval.contains(&mem)
    }

    fn zadeh_operation<F>(&self, other: &FuzzySet<T>, f: F) -> FuzzySet<T>
    where F: Fn(f32, f32) -> f32 { 
        let elems: Vec<(T, f32)> = vec![];
        let mut new_set = FuzzySet::new(elems, None);

        for val in &self.elements {
            new_set.elements.push(*val)
        }

        for (el, mem) in &other.elements {
            if let Some(existing_membership) = self.get_elem_membership(*el) {
                let new_mem = f(existing_membership, *mem);
                new_set.change_elem_membership(*el, new_mem);
            } else {
                new_set.add((*el, *mem));
            }
        }

        new_set
    }
}