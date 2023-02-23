use std::ops::IndexMut;
use crate::asm::class::Visitor;

pub(crate) trait Replacable<T>: IndexMut<usize, Output = T> {}

impl<T> Replacable<T> for Vec<T> {}

impl<T, const N: usize> Replacable<T> for [T; N] {}

pub(crate) fn replace<T>(
    start_index: usize,
    dest: &mut impl Replacable<T>,
    items: impl IntoIterator<Item = T>,
    size: usize,
) where
    T: Copy,
{
    let mut item_iter = items.into_iter();

    for i in 0..size {
        if let Some(item) = item_iter.next() {
            let _ = std::mem::replace(&mut dest[start_index + i], item);
        } else {
            break;
        }
    }
}

pub(crate) trait Rev<const N: usize> {
    fn as_rev(&self) -> [u8; N];
}

impl Rev<4> for i32 {
    fn as_rev(&self) -> [u8; 4] {
        let mut bytes = self.to_ne_bytes();
        bytes.reverse();
        bytes
    }
}

trait Delegated<T> where T: Delegated<T> {
    fn delegated(&self) -> &T;
}

impl<V> Delegated<V> for V where V: Visitor {
    fn delegated(&self) -> &V {
        self
    }
}

#[cfg(test)]
mod test {
    use super::replace;

    #[test]
    pub fn test_replace() {
        let mut buffer = vec![1, 2, 3, 4, 5];
        let arr = [100, 200, 300];

        replace(1, &mut buffer, arr.into_iter(), 2);

        assert_eq!(buffer, vec![1, 100, 200, 4, 5]);
    }
}
