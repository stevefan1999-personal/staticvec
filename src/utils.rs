use crate::StaticVec;
use core::cmp::{Ordering, PartialOrd};
use core::intrinsics;
use core::mem::MaybeUninit;

#[inline(always)]
pub(crate) const fn distance_between<T>(dest: *const T, origin: *const T) -> usize {
  match intrinsics::size_of::<T>() {
    0 => unsafe { (dest as usize).wrapping_sub(origin as usize) },
    // Safety: this function is used strictly with linear inputs
    // where dest is known to come after origin.
    _ => unsafe { intrinsics::ptr_offset_from(dest, origin) as usize },
  }
}

#[inline]
pub(crate) fn reverse_copy<T, const N: usize>(this: &MaybeUninit<[T; N]>) -> MaybeUninit<[T; N]>
where T: Copy {
  let mut res: MaybeUninit<[T; N]> = MaybeUninit::uninit();
  let src = this.as_ptr() as *const T;
  let mut dest = res.as_mut_ptr() as *mut T;
  let mut i = N;
  while i > 0 {
    unsafe {
      src.add(i - 1).copy_to_nonoverlapping(dest, 1);
      dest = dest.offset(1);
      i -= 1;
    }
  }
  res
}

#[inline(always)]
pub fn new_from_value<T, const COUNT: usize>(value: T) -> StaticVec<T, COUNT>
where T: Copy {
  StaticVec {
    data: {
      unsafe {
        let mut data = StaticVec::new_data_uninit();
        for i in 0..COUNT {
          (data.as_mut_ptr() as *mut T).add(i).write(value);
        }
        data
      }
    },
    length: COUNT,
  }
}

#[inline]
pub(crate) fn partial_compare<T1, T2: PartialOrd<T1>>(
  this: &[T2],
  other: &[T1],
) -> Option<Ordering>
{
  let min_length = this.len().min(other.len());
  unsafe {
    let left = this.get_unchecked(0..min_length);
    let right = other.get_unchecked(0..min_length);
    for i in 0..min_length {
      match left.get_unchecked(i).partial_cmp(right.get_unchecked(i)) {
        Some(Ordering::Equal) => (),
        non_eq => return non_eq,
      }
    }
  }
  this.len().partial_cmp(&other.len())
}
