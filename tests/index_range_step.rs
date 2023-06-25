use iter_cyclic::range_step_idx;

#[test]
fn one_step_full() {
    assert_eq!((0..255).collect::<Vec<_>>(), range_step_idx(0, 254, 255, 255).collect::<Vec<_>>());
}

#[test]
fn step_step_by() {
    assert_eq!((0..255).step_by(5).collect::<Vec<_>>(), range_step_idx(0, 0, 5, 255).collect::<Vec<_>>());
}

#[test]
fn start_gt_stop() {
    assert_eq!((1..0).collect::<Vec<_>>(), range_step_idx(1, 0, 5, 255).collect::<Vec<_>>());
}

#[test]
fn stop_gt_step() {
    assert_eq!((1..0).collect::<Vec<_>>(), range_step_idx(0, 6, 5, 255).collect::<Vec<_>>());
}

#[test]
fn step_gt_end() {
    assert_eq!((1..0).collect::<Vec<_>>(), range_step_idx(0, 6, 256, 255).collect::<Vec<_>>());
}

#[test]
fn step_eq_end() {
    assert_eq!((0..255).collect::<Vec<_>>(), range_step_idx(0, 254, 255, 255).collect::<Vec<_>>());
}


