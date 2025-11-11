use std::cmp::min;

pub fn as_raw_parts<T>(s: &[T]) -> (*const T, usize) {
    (s.as_ptr(), s.len())
}

pub fn as_raw_mut_parts<T>(s: &mut [T]) -> (*mut T, usize) {
    (s.as_mut_ptr(), s.len())
}

pub fn slice_copy_min_len<T: Copy>(input: &[T], output: &mut [T]) -> usize {
    let len = min(input.len(), output.len());
    let input = &input[..len];
    let output = &mut output[..len];
    output.copy_from_slice(input);
    len
}
