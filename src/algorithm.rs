use chromaprint_sys as sys;
/// The different fingerprint algorithms available.
///
/// Defaults to `Algorithm::Test2`
pub enum Algorithm {
    Test1,
    Test2,
    Test3,
    Test4,
    Test5,
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Test2
    }
}

impl From<sys::ChromaprintAlgorithm> for Algorithm {
    fn from(algo: sys::ChromaprintAlgorithm) -> Self {
        match algo {
            0 => Algorithm::Test1,
            1 => Algorithm::Test2,
            2 => Algorithm::Test3,
            3 => Algorithm::Test4,
            4 => Algorithm::Test5,
            _ => Algorithm::Test2
        }
    }
}
impl From<i32> for Algorithm {
    fn from(algo: i32) -> Self {
        match algo {
            0 => Algorithm::Test1,
            1 => Algorithm::Test2,
            2 => Algorithm::Test3,
            3 => Algorithm::Test4,
            4 => Algorithm::Test5,
            _ => Algorithm::Test2
        }
    }
}

impl Into<sys::ChromaprintAlgorithm> for Algorithm {
    fn into(self) -> sys::ChromaprintAlgorithm {
        match self {
            Algorithm::Test1 => 0,
            Algorithm::Test2 => 1,
            Algorithm::Test3 => 2,
            Algorithm::Test4 => 3,
            Algorithm::Test5 => 4,
        }
    }
}

impl Into<i32> for Algorithm {
    fn into(self) -> i32 {
        match self {
            Algorithm::Test1 => 0,
            Algorithm::Test2 => 1,
            Algorithm::Test3 => 2,
            Algorithm::Test4 => 3,
            Algorithm::Test5 => 4,
        }
    }
}
