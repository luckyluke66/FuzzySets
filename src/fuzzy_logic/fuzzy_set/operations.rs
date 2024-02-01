use super::set::FuzzySet;

pub trait FuzzyOperations<T> {
    fn union(&self, other: &FuzzySet<T>) -> FuzzySet<T>;

    fn intersection(&self, other: &FuzzySet<T>) -> FuzzySet<T>;

    fn implication(&self, other: &FuzzySet<T>) -> FuzzySet<T>;

    fn defuzzyfication(&self, other: &FuzzySet<T>) -> FuzzySet<T>;
}

impl<T: PartialOrd + Copy> FuzzyOperations<T> for FuzzySet<T>  {
    fn union(&self, other: &FuzzySet<T>) -> FuzzySet<T> {
        self.zadeh_operation(other, 
            |x: f32, y: f32| x.max(y))
    }

    fn intersection(&self, other: &FuzzySet<T>) -> FuzzySet<T> {
        self.zadeh_operation(other, 
            |x: f32, y: f32| x.min(y))
    }

    fn implication(&self, other: &FuzzySet<T>) -> FuzzySet<T> {
        //TODO: implement function
        FuzzySet::new(vec![], None)
    }

    fn defuzzyfication(&self, other: &FuzzySet<T>) -> FuzzySet<T> {
        FuzzySet::new(vec![], None)
        //TODO: implement function
    }
}

impl<T: PartialOrd + Copy> FuzzySet<T> {
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