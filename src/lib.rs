fn get_rand_bytes(s: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; s];
    rand::fill(&mut buffer);
    buffer
}

#[test]
fn test_get_rand_bytes() {
    const SIZE: usize = 1024;
    let result = get_rand_bytes(SIZE);
    assert_eq!(result.len(), SIZE);
    assert_eq!(std::mem::size_of_val(&*result), SIZE);
}

fn allocate_empty(s: usize) -> Vec<u8> {
    let buffer = vec![0u8; s];
    buffer
}

#[test]
fn test_allocate_empty() {
    const SIZE: usize = 1024;
    let result = allocate_empty(SIZE);
    assert_eq!(result.len(), SIZE);
    assert_eq!(std::mem::size_of_val(&*result), SIZE);
}

pub fn thread_worker(size: usize, count: usize, random_value: bool) -> Vec<Vec<u8>> {
    let mut hogged = vec![];

    #[cfg(debug_assertions)]
    println!("Thread block count: {count}");

    for _ in 0..count {
        if random_value {
            hogged.push(get_rand_bytes(size));
        } else {
            hogged.push(allocate_empty(size))
        }
    }
    hogged
}

pub fn get_hogged_size(hogged: &Vec<Vec<u8>>) -> usize {
    let mut hogged_size = 0;
    for block_ref in hogged {
        let block_size = std::mem::size_of_val(&**block_ref);
        hogged_size += block_size;
        hogged_size += std::mem::size_of_val(block_ref);
    }
    hogged_size += std::mem::size_of_val(hogged);

    hogged_size
}
