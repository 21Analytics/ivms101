mod constrained_string;
mod non_empty_vec;
pub(crate) mod one_to_n;
pub(crate) mod zero_to_n;

use crate::Error;
crate::constrained_string!(StringMax16, |l| l <= 16);
crate::constrained_string!(StringMax35, |l| l <= 35);
crate::constrained_string!(StringMax50, |l| l <= 50);
crate::constrained_string!(StringMax70, |l| l <= 70);
crate::constrained_string!(StringMax100, |l| l <= 100);
