use crate::crisp_set::Set;
use super::set::FuzzySet;

impl<T: Ord + Copy> FuzzySet<T> {
    pub fn height(&self) -> f32 {
        let mut max = 0.0;
        
        for mem in self.memberships() {
            if mem > max {
                max = mem;
            }
        }
        max
    }

    pub fn height2(&self) -> f32 {
        let max_mem = self.memberships().iter().
            fold(0.0, |x: f32, &acc| x.max(acc));
        max_mem
    }

    pub fn support(&self) -> Set<T> {
        self.create_set(|&(_, mem)| mem > 0.0)
    }
    
    pub fn kernel(&self) -> Set<T> {
        self.create_set(|&(_, mem)| mem == 1.0)
    }
    
    pub fn alpha_cut(&self, border_mem: f32) -> Set<T> {
        self.create_set(|&(_, mem)| mem > border_mem)
    }
    
    fn create_set<P>(&self, pred: P) -> Set<T>
    where
        P: Fn(&(T, f32)) -> bool,
    {
        let elems: Vec<T> = self.elements.iter()
            .filter(|&pair| pred(pair))
            .map(|&(el, _)| el).collect();
        Set::new(elems)
    }
}