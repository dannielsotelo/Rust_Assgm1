// Copyright © 2019 Danniel Sotelo
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
///Referenced this two sites to help me create this function
///https://stackoverflow.com/questions/45309240/how-do-i-perform-operations-on-different-numeric-types-while-computing-the-avera
///https://benjaminbrandt.com/averages-in-rust/
pub fn mean(nums: &[f64]) -> Option<f64> {
    let mut avg = 0f64;
    if nums.len() < 1 {
        Some(avg)
    }else{
        //iterates through nums vecotr and sums up all the elements
        let sum: f64 = Iterator::sum(nums.iter());
        let length = nums.len() as f64;
        avg = sum / length;
        Some(avg)
    }
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
///http://xion.io/post/code/rust-for-loop.html
pub fn stddev(nums: &[f64]) -> Option<f64> {
    if nums.len() < 2 {
        None
    } else {
        let average = mean(&nums);
        let mut variance = Vec::new();
        for i in nums {
            let i = (i - average.unwrap()).powf(2f64);
            variance.push(i);
        }
        mean(&variance)
    }
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
///Logic for median: If nums list is odd, median is the center of a sorted Nums list. 
///If nums list is even, add the two middle terms and divide by 2.
///Referenced:
///https://benjaminbrandt.com/averages-in-rust/
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if nums.len() < 1 {
        None
    }else {
        let middle = (nums.len() / 2) as f64;
        if nums.len() % 2 == 0 {
            Some(nums[(middle-1f64)as usize])
        } else {
        Some(middle)
        }
    }
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
///https://www.dcode.fr/vector-norm
///used Iterator:: from mean() above
pub fn l2(nums: &[f64]) -> Option<f64> {
    if nums.len() == 0 {
        Some(0.0f64)
    } else {
        let mut nums_sqr = Vec::new();
        for i in nums {
            let i = i.powf(2f64);
            nums_sqr.push(i);
        }
        let sum: f64 = Iterator::sum(nums_sqr.iter());
        Some(sum.sqrt())
    }
}
