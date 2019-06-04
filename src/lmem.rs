use crate::util::pod::Pod;
use crate::util::data::{d8};
use core::{mem, ptr, slice};

/// Creates a sequence of zero bytes.
///
/// [return_value]
/// A value that contains a sequence of zero bytes.
pub fn zeroed<T>() -> T
    where T: Pod,
{
    unsafe { mem::zeroed() }
}

/// Returns an unspecified byte sequence of a value.
///
/// [argument, val]
/// The value.
///
/// [return_value]
/// An unspecified byte sequence of the value.
///
/// = Description
///
/// Each byte in the returned slice can be undefined for its type.
pub fn as_data<T: ?Sized>(val: &T) -> &[d8] {
    unsafe { slice::from_raw_parts(val as *const _ as *const _, mem::size_of_val(val)) }
}

/// Returns an unspecified mutable byte sequence of a value.
///
/// [argument, val]
/// The value.
///
/// [return_value]
/// An unspecified mutable byte sequence of the value.
///
/// = Description
///
/// Each byte in the returned slice can be undefined for its type.
pub fn as_mut_data<T: ?Sized>(val: &mut T) -> &mut [d8]
    where T: Pod,
{
    unsafe { slice::from_raw_parts_mut(val as *mut _ as *mut _, mem::size_of_val(val)) }
}

/// Returns whether a buffer is suitable to hold an object of a certain type.
///
/// [argument, buf]
/// The buffer to be checked.
///
/// = Remarks
///
/// The buffer is suitable if it is large enough to hold the type and properly aligned.
pub fn is_suitable_for<T>(buf: &[u8]) -> bool {
    (buf.len() >= mem::size_of::<T>()) && (buf.as_ptr() as usize & (mem::align_of::<T>() - 1) == 0)
}

/// Turns a slice into a reference to a Pod type if it's suitable.
///
/// [argument, buf]
/// The buffer to be turned into a reference.
///
/// [return_value]
/// Returns the created reference.
///
/// = Remarks
///
/// The buffer is suitable under the conditions described in
/// link:lrs::mem::is_suitable_for[is_suitable_for].
pub fn from_bytes<T>(buf: &[u8]) -> Option<&T>
    where T: Pod,
{
    match is_suitable_for::<T>(buf) {
        true => Some(unsafe { &*(buf.as_ptr() as *const T) }),
        _ => None,
    }
}

/// Turns a mutable slice into a mutable reference to a Pod type if it's suitable.
///
/// [argument, buf]
/// The buffer to be turned into a reference.
///
/// [return_value]
/// Returns the created reference.
///
/// = Remarks
///
/// The buffer is suitable under the conditions described in
/// link:lrs::mem::is_suitable_for[is_suitable_for].
pub fn from_mut_bytes<T>(buf: &mut [u8]) -> Option<&mut T>
    where T: Pod,
{
    match is_suitable_for::<T>(buf) {
        true => Some(unsafe { &mut *(buf.as_mut_ptr() as *mut T) }),
        _ => None,
    }
}

/// Copies an object and casts the result to another type.
///
/// [argument, val]
/// The object to be copied.
///
/// = Remarks
///
/// `T` and `U` can have different sizes but if the size of `U` is larger than `T` and
/// reading from the trailing bytes causes invalid memory access, the behavior is
/// undefined.
pub unsafe fn copy_as<T, U>(src: &T) -> U {
    ptr::read(src as *const T as *const U)
}

/// Drops a value.
///
/// [argument, _val]
/// The object to be dropped.
pub fn drop<T>(_val: T) { }

/// Copies bytes from one slice to another.
///
/// [argument, dst]
/// The slice in which the objects will be stored.
///
/// [argument, src]
/// The slice from which the objects will be copied.
///
/// [return_value]
/// Returns the number of objects copied.
///
/// = Remarks
///
/// The number of entries copied is the minimum length of both slices.
pub fn copy<T: Copy>(dst: &mut [T], src: &[T]) -> usize {
    unsafe { unsafe_copy(dst, src) }
}

/// Copies bytes from one slice to another even if the type does not implement `Copy`.
///
/// [argument, dst]
/// The slice in which the objects will be stored.
///
/// [argument, src]
/// The slice from which the objects will be copied.
///
/// [return_value]
/// Returns the number of objects copied.
///
/// = Remarks
///
/// The number of entries copied is the minimum length of both slices.
pub unsafe fn unsafe_copy<T>(dst: &mut [T], src: &[T]) -> usize {
    let min = dst.len().min(src.len());
    ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), min);
    min
}

/// Swaps two objects.
///
/// [argument, one]
/// Object one.
///
/// [argument, two]
/// Object two.
pub fn swap<T>(one: &mut T, two: &mut T) {
    unsafe {
        let tmp: T = copy_as(one);
        ptr::copy_nonoverlapping(two, one, 1);
        ptr::write(two, tmp)
    }
}

/// Turns a reference into a one-element slice.
///
/// [argument, val]
/// The object that will be the element of the slice.
pub fn as_slice<T>(val: &T) -> &[T] {
    unsafe { slice::from_raw_parts(val as *const _, 1) }
}

/// Turns a mutable reference into a mutable one-element slice.
///
/// [argument, val]
/// The object that will be the element of the slice.
pub fn as_mut_slice<T>(val: &mut T) -> &mut [T] {
    unsafe { slice::from_raw_parts_mut(val as *mut _, 1) }
}

/// Left-trims a byte slice so that the first element is aligned.
///
/// [argument, buf]
/// The slice to be trimmed.
///
/// = Remarks
///
/// That is, if the returned slice is not empty, the address of the first element is a
/// multiple of the alignment of `T`.
pub fn align_for<T>(buf: &[u8]) -> &[u8] {
    let align_mask = mem::align_of::<T>() - 1;
    let addr = buf.as_ptr() as usize;
    let diff = ((!addr & align_mask) + 1) & align_mask;
    if diff <= buf.len() {
        &buf[diff..]
    } else {
        &[]
    }
}

/// Left-trims a mutable byte slice so that the first element is aligned.
///
/// [argument, buf]
/// The slice to be trimmed.
///
/// = Remarks
///
/// That is, if the returned slice is not empty, the address of the first element is a
/// multiple of the alignment of `T`.
pub fn align_for_mut<T>(buf: &mut [u8]) -> &mut [u8] {
    let align_mask = mem::align_of::<T>() - 1;
    let addr = buf.as_ptr() as usize;
    let diff = ((!addr & align_mask) + 1) & align_mask;
    if diff <= buf.len() {
        &mut buf[diff..]
    } else {
        &mut []
    }
}

/// Returns the address of an object.
///
/// [argument, obj]
/// The object whose address will be returned.
pub fn addr<T>(obj: &T) -> usize {
    obj as *const T as usize
}
