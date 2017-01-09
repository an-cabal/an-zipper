use ::{List, Stack};
use quickcheck::{Arbitrary, Gen};

impl<T> Arbitrary for List<T>
where T: Arbitrary {

    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Vec::<T>::arbitrary(g).into_iter().collect()
    }

    fn shrink(&self) -> Box<Iterator<Item=List<T>>> {
        // Shrink a list by shrinking a vector of its elements
        let elems: Vec<T> = self.iter().cloned().collect();
        Box::new(elems.shrink().map(|x| x.into_iter().collect::<List<T>>()))
    }

}

quickcheck! {
    fn push_and_pop_same_item(list: List<usize>, item: usize) -> bool {
        let mut list = list;
        list.push(item);
        list.pop() == Some(item)
    }
    fn push_and_pop_2_same_items(list: List<usize>, item1: usize, item2: usize)
                                -> bool {
        let mut list = list;
        list.push(item1);
        list.push(item2);
        list.pop() == Some(item2) && list.pop() == Some(item1)
    }
    fn push_and_pop_n_same_items(list: List<usize>, items: Vec<usize>)
                                -> bool {
        let mut list = list;

        for item in items.clone() {
            list.push(item);
        }

        items.iter().rev().all(|item| list.pop() == Some(*item))
    }

    fn push_and_peek_same_item(list: List<usize>, item: usize) -> bool {
        let mut list = list;
        list.push(item);
        list.peek() == Some(&item)
    }
    fn push_and_peek_2_same_item(list: List<usize>, item1: usize, item2: usize)
                                -> bool {
        let mut list = list;
        list.push(item1);
        list.push(item2);
        list.peek() == Some(&item2) && list.peek() == Some(&item2)
    }
    fn push_and_peek_n_same_items(list: List<usize>, items: Vec<usize>)
                                -> bool {
        let mut list = list;

        for item in items.clone() {
            list.push(item);
        }

        (0..items.len()).all(|_| list.peek() == items.last())
    }

    fn iter_in_order(items: Vec<usize>) -> bool {
        let mut list = List::new();

        for item in items.clone() {
            list.push(item);
        }

        let result = list.iter().collect::<Vec<_>>();
        let reversed = items.iter().rev().collect::<Vec<_>>();
        result == reversed
    }

    fn iter_mut_in_order(items: Vec<usize>) -> bool {
        let mut list = List::new();

        for item in items.clone() {
            list.push(item);
        }

        for mut item in list.iter_mut() {
            *item = *item + 1;
        }

        list.iter().zip(items.iter().rev())
            .all(|(a, b)| *a == b + 1)
    }

    fn move_iter_in_order(items: Vec<usize>) -> bool {
        let mut list = List::new();

        for item in items.clone() {
            list.push(item);
        }

        list.into_iter().zip(items.iter().rev())
            .all(|(a, b)| &a == b)
    }
}
