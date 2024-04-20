use approx_eq::assert_approx_eq;
use vec_utilities::running::Running;

#[test]
fn test_running_sum() {
    let v = vec![1.0, 2.0, 3.0];

    let running = v.iter().running_sum();

    assert_eq!(running, vec![1.0, 3.0, 6.0]);
}

#[test]
fn test_running_mean() {
    let v = vec![1.0, 2.0, 3.0, 4.0];

    let running = v.iter().running_mean();

    assert_eq!(running, vec![1.0, 1.5, 2.0, 2.5]);
}

#[test]
fn test_running_std() {
    let v = vec![1.0, 2.0, 3.0, 4.0];

    let running = v.iter().running_std();
    let correct = vec![0.0, 0.70710678118655, 1.0, 1.2909944487358];
    println!("std {:?}", running);

    for (r, c) in running.iter().zip(correct.iter()) {
        assert_approx_eq!(*r, *c, 1e-8);
    }
}
