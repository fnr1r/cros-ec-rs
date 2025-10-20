pub fn as_raw_parts<T>(s: &[T]) -> (*const T, usize) {
    (s.as_ptr(), s.len())
}

pub fn as_raw_mut_parts<T>(s: &mut [T]) -> (*mut T, usize) {
    (s.as_mut_ptr(), s.len())
}
