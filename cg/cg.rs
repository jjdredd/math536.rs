use std::vec::Vec;
use std::clone::Clone

struct MCoor {
    row : usize,
    col : usize,
}

pub struct CSM {
    usize N,
    Vec<Mcoor> C,
    Vec<f64> Elem,
}

impl CSM {
    // construct a new CSM
    pub fn new(Size : usize) -> CSM {
        CSM {
            N : Size,
            C : Vec::new(),
            Elem : Vec::new(),
        }
    }
    // construct from another CSM
    pub fn new(A : CSM) -> CSM {
        CSM {
            N : A.N,
            C : A.C.clone(),
            Elem : A.Elem.clone(),
        }
    }
    // return size of vectors
    pub fn Size(&self) -> usize {
        self.N
    }
    // Get an element
    pub fn Get(&self, m, n) -> f64 {

        assert!(m < N && n < N,
                "Get: row {} or column {} out of range {}", m, n, N);

        for ind in self.C.len() {
            if m == self.C[ind].row && n == self.C[ind].col {
                return self.Elem[ind];
            }
        }
        return 0;
    }
    // Set an element
    pub fn Set (&mut self, m, n, val : f64) {

        assert!(m < N && n < N,
                "Set: row {} or column {} out of range {}", m, n, N);

        self.C.push(MCoor {row : m, col : n});
        self.Elem.push(val);
    }
}


// clone trait
impl Clone for CSM {
    pub clone(&self) -> CSM {
        CSM {
            N : self.N,
            C : self.C.clone(),
            Elem : self.Elem.clone(),
        }
    }

    pub clone_from(&mut self, source: &Self) {
        // Performs copy-assignment from source.

        // a.clone_from(&b) is equivalent to a = b.clone()
        // in functionality, but
        // can be overridden to reuse the resources of
        // a to avoid unnecessary allocations.

    }
}
