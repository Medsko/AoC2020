pub fn find_combinations(numbers: Vec<u32>,
                     current: &mut Vec<u32>,
                     combinations: &mut Vec<Vec<u32>>,
                     size: usize) {

    for index in 0..numbers.len() {
        let mut available_numbers = numbers.clone();
        current.push(available_numbers[index]);
        available_numbers.drain(0..index);

        if current.len() == size {
            combinations.push(current.clone());
            current.remove(size - 1);
            continue;
        }
        find_combinations(available_numbers, current, combinations, size);
        current.remove(current.len() - 1);
    }
}

#[test]
fn all_combinations_found() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    let mut current: Vec<u32> = Vec::new();
    let mut combinations: Vec<Vec<u32>> = Vec::new();
    find_combinations(input, &mut current, &mut combinations, 3);

    assert_eq!(combinations.len(), 20);
}