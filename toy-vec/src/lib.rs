#[cfg(test)]
mod tests {
    use super::ToyVec;
    #[test]
    fn it_works() {
        let mut v = ToyVec::<String>::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());
        let e = v.get(0);
        assert_eq!(e, Some(&"Java Finch".to_string()));
    }

    #[test]
    fn iter() {
        let mut v = ToyVec::<String>::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());

        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
        assert_eq!(iter.next(), Some(&"Budgerigar".to_string()));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        for (i, s) in v.iter().enumerate() {
            assert_eq!(s, v.get(i).unwrap());
        }
    }
}

pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // 可変借用から所有権を奪えないので、これは駄目
            // let element = self.elements[self.len];

            let element = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(element)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    // デフォルト値とレシーバのライフタイムを明示的に揃えておかないと、matchで戻す戻り値のライフタイムが揃えられない
    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
    }

    fn grow(&mut self) {
        if self.len == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.len * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            // Sliceの into_iter は &Tのイテレータを返してしまうのでNG
            // for (i, e) in old_elements.into_iter().enumerate() {
            //     self.elements[i] = e;
            // }

            // SliceからVecに変換したあと、Vecの into_iter を使うと T のイテレータが手に入る
            for (i, e) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = e;
            }
        }
    }

    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl <'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.len {
            None
        } else {
            self.pos += 1;
            Some(&self.elements[self.pos - 1])
        }
    }
}
