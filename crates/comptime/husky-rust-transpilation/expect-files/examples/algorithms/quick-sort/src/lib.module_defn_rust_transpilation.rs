#![allow(warnings, non_snake_case)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};



#[allow(non_upper_case_globals)]
pub static mut __quick_sort__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn quick_sort<T: Ord>(ref mut arr: &mut [T]) {
    let len = arr.len();
    crate::quick_sort_aux(arr, 0, (len - 1) as isize)
}

#[allow(non_upper_case_globals)]
pub static mut __quick_sort_aux__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn quick_sort_aux<T: Ord>(ref mut arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = crate::partition(arr, low, high);
        crate::quick_sort_aux(arr, low, p - 1);
        crate::quick_sort_aux(arr, p + 1, high)
    }
}

#[allow(non_upper_case_globals)]
pub static mut __partition__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn partition<T: Ord>(ref mut arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;
    while true {
        store_index += 1;
        while &arr[store_index as usize as usize] < &arr[pivot as usize] {
            store_index += 1
        }
        last_index -= 1;
        while last_index >= 0 && &arr[last_index as usize as usize] > &arr[pivot as usize] {
            last_index -= 1
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize)
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

#[allow(non_upper_case_globals)]
pub static mut __quick_sort_works_for_integers__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn quick_sort_works_for_integers() {
    let mut v: Vec<i32> = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
}

#[allow(non_upper_case_globals)]
pub static mut __quick_sort_works_for_strs__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn quick_sort_works_for_strs() {
    let mut strs = vec!["beach", "hotel", "airplane", "car", "house", "art"];
}