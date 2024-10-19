use crate::customer::Customer;

// This is just a basic binary search algorithm
pub fn binary_search(arr: &[Customer], target: &Customer) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;

    while left <= right {
        let mid = (left + right) / 2;

        match arr[mid as usize].amount_spent.cmp(&target.amount_spent) {
            std::cmp::Ordering::Less => {
                left = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                right = mid - 1;
            }
            std::cmp::Ordering::Equal => {
                return Some(mid as usize);
            }
        }
    }
    None
}
