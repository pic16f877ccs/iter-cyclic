use iter_cyclic::RangeStepVec;

#[test]
fn empty_vector() {
    let vec: Vec<char> = Vec::new();
    assert_eq!(vec.range_step_iter(0, 3, 5).collect::<Vec<_>>(), Vec::<char>::new());
}

#[test]
fn step_by_five() {
    let vec = (0..=254).collect::<Vec<u8>>();
    assert_eq!(vec.range_step_iter(0, 0, 5).collect::<Vec<_>>(), (0..=254).step_by(5).collect::<Vec<u8>>());
}

#[test]
fn start_gt_stop() {
    let vec = vec![0u8; 100];
    assert_eq!(vec.range_step_iter(1, 0, 5).collect::<Vec<_>>(), Vec::<u8>::new());
}

#[test]
fn stop_gt_step() {
    let vec = vec![0u8; 100];
    assert_eq!(vec.range_step_iter(0, 6, 5).collect::<Vec<_>>(), Vec::<u8>::new());
}

#[test]
fn step_gt_vec_len() {
    let vec = vec![0u8; 255];
    assert_eq!(vec.range_step_iter(0, 6, 256).collect::<Vec<_>>(), Vec::<u8>::new());
}

#[test]
fn one_full_step() {
    let vec = (0..=254).collect::<Vec<u8>>();
    assert_eq!(vec.clone().range_step_iter(0, 254, 255).collect::<Vec<_>>(), vec);
}

#[test]
fn range_two_step_four() {
    let vec = (0..12).collect::<Vec<u8>>();
    assert_eq!(vec.range_step_iter(0, 1, 4).collect::<Vec<_>>(), vec![0, 1, 4, 5, 8, 9u8]);
}

#[test]
fn range_step_neg() {
    let vec: Vec<i8> = (-10..24).collect();
    assert_eq!(vec.range_step_iter(0, 2, 7).collect::<Vec<_>>(), vec![-10, -9, -8, -3, -2, -1, 4, 5, 6, 11, 12, 13]);
}

#[test]
fn range_step_change_vec_value() {
    let mut vec: Vec<u8> = (0..19).collect();
    vec.range_step_value(0, 2, 7, 10);
    assert_eq!(vec, [10, 10, 10, 3, 4, 5, 6, 10, 10, 10, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
}

#[test]
fn range_step_change_vec_values() {
    let mut vec: Vec<u8> = (0..=21).collect();
    vec.range_step_values(0, 2, 7, 0..100);
    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6, 3, 4, 5, 10, 11, 12, 13, 6, 7, 8, 17, 18, 19, 20, 21]);
}

#[test]
fn range_step_new_vec() {
    let vec: Vec<i8> = (-10..=25).collect();
    let new_vec = vec.range_step_vec(0, 2, 7);
    assert_eq!(new_vec, [-10, -9, -8, -3, -2, -1, 4, 5, 6, 11, 12, 13, 18, 19, 20]);
}

#[test]
fn range_step_char_vec() {
    let vec: Vec<char> = "lisinumbx!de".chars().collect();
    let new_vec = vec.range_step_vec(0, 1, 4);
    assert_eq!(new_vec, ['l', 'i', 'n', 'u', 'x', '!']);
}
