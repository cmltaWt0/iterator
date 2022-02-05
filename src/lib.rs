use std::ops::AddAssign;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt;


pub struct Stepper<T> {
    curr: T,
    step: T,
    stop: T,
}

impl<T> Stepper<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        Stepper {
            curr: start,
            stop: stop,
            step: step,
        }
    }
}

impl<T> Iterator for Stepper<T>
    where T: AddAssign + Copy + PartialOrd
{
    type Item=T;

    fn next(&mut self) -> Option<T> {
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn sum_list<I, S>(l: I, mut s: S) -> S
    where I: IntoIterator<Item=S>,
          S: AddAssign,
{
    let mut it = l.into_iter();
    while let Some(n) = it.next() {
        s += n;
    }
    s
    /* for n in l {
        s += n;
    }
    s*/
}


#[derive(Debug, PartialEq)]
pub struct USD(i32);


impl Display for USD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (self.0 as f32) / 100.;
        if r < 0. {
            return write!(f, "-${:.2}", -r);
        }
        write!(f, "${:.2}", r)
    }
}

impl Clone for USD {
    fn clone(&self) -> Self {
        USD(self.0)
    }
}

impl Copy for USD {

}



#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let mut c = 0;
        for n in Stepper::new(2, 10, 2)  {
            c += n;
        }
        assert_eq!(c, 20);

        let s = Stepper::new(2, 12, 3);
        assert_eq!(sum_list(s, 0), 26);

        let fl = Stepper::new(4, 10, 2).fold(0, |acc, x| acc + x);
        assert_eq!(fl, 18);
    }

    #[test]
    fn test_trait() {
        let u = USD(230);
        assert_eq!(u.to_string(), "$2.30".to_string());

        let b = u;
        assert_eq!(b, u);
    }
}
