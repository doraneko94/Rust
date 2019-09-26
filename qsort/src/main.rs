fn partition<T: Ord>(slice: &[T]) -> usize {
    let length = slice.len();
    if length % 2 == 0 {
        return length / 2;
    } else {
        return (length + 1) / 2;
    }
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let pivot_index = partition(slice);

    quicksort(&mut slice[.. pivot_index]);

    quicksort(&mut slice[pivot_index ..]);
}

fn main() {
    let mut v: Vec<i32> = vec![5, 2, 3, 4, 1];
    quicksort(&mut v);
    for i in &v {
        println!("{}", &i);
    }
}
