use iter_cyclic::RangeStepIter;

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
