use std::vec::Vec;
use std::clone::Clone;
use std::ops::Mul;

struct MCoor {
    row : usize,
    col : usize,
}

pub struct CSM {
    N : usize,
    C : Vec<MCoor>,
    Elem : Vec<f64>,
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
    pub fn new_from(A : &CSM) -> CSM {
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
    pub fn Get(&self, m : usize, n : usize) -> f64 {

        assert!(m < self.N && n < self.N,
                "Get: row {} or column {} out of range {}", m, n, self.N);

        for ind in self.C.len() {
            if m == self.C[ind].row && n == self.C[ind].col {
                return self.Elem[ind];
            }
        }
        return 0;
    }
    // Set an element
    pub fn Set (&mut self, m : usize, n : usize, val : f64) {

        assert!(m < self.N && n < self.N,
                "Set: row {} or column {} out of range {}", m, n, self.N);

        self.C.push(MCoor {row : m, col : n});
        self.Elem.push(val);
    }
}


// clone trait
impl Clone for CSM {
    pub fn clone(&self) -> CSM {
        CSM {
            N : self.N,
            C : self.C.clone(),
            Elem : self.Elem.clone(),
        }
    }

    pub fn clone_from(&mut self, source: &CSM) {
        // Performs copy-assignment from source.

        // a.clone_from(&b) is equivalent to a = b.clone()
        // in functionality, but
        // can be overridden to reuse the resources of
        // a to avoid unnecessary allocations.

        self.N = source.N;
        self.C = source.C.clone();
        self.Elem = source.Elem.clone();
    }
}


// overload multiplication
impl Mul for CSM {

    type Output = Vec<f64>;

    pub fn mul(&self, rhs : &Vec<f64>) -> Vec<f64> {

        let N = self.Size();
        assert!(N == rhs.len(),
                "size mismatch in fn mul(&self, rhs : &Vec<f64>) -> Vec<f64>");

        let v = vec![0 as f64; N as usize];
 
        for i in 0..N {
            for ind in self.C.len() {
                if self.C[ind].row == i {
                    v[i] += self.Elem[ind] * rhs[self.C[ind].col];
                }
            }
        }
        v
    }
}

impl Mul for Vec<f64> {

    type Output = Vec<f64>;

    pub fn mul(&self, rhs : &CSM) -> Vec<f64> {

        let N = rhs.Size();
        assert!(self.len() == N,
                "size mismatch in fn mul(&self, rhs : &CSM) -> Vec<f64>");
        let v = vec![0 as f64; N as usize];

        for i in 0..N {
            for ind in rhs.C.len() {
                if rhs.C[ind].row == i {
                    v[i] += rhs.Elem[ind] * self[rhs.C[ind].col];
                }
            }
        }
        v
    }
}

impl Mul for Vec<f64> {

    type Output = f64;

    pub fn mul(&self, rhs : &Vec<f64>) -> f64 {

        let N = self.len();

        assert!(N == rhs.len(),
                "size mismatch in pub fn mul(&self, rhs : &Vec<f64>) -> f64 {");

        let r : f64 = 0;
        for i in 0..N {
            r += self[i] * rhs[i];
        }
        r
    }
}

pub fn AProd(A : &CSM, a : &Vec<f64>, b : &Vec<f64>) -> f64 {
    a * (A * b)
}

pub fn ANorm(A : &CSM, a : &Vec<f64>) -> f64 {
    a * (A * a)
}

// pub fn CG (A : &CSM, b : &Vec<f64>, steps : &u32, e : f64) -> Vec<f64> {

//     let N = b.len();
//     let a : f64, error : f64;
//     let r : Vec<f64>, p : Vec<f64>, u = vec![0 as f64; N as usize];

//     steps = 0;
//     p = r = b
// }
