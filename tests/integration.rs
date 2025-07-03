use crop_tracking_system::tracker::{Tracker};
use crop_tracking_system::detection::Detection;

#[test]
fn test_full_tracker_workflow() {
    let mut tracker = Tracker::new(3, 0.3);
    let dets = vec![
        Detection::new( 0.3, 0.3, 0.1, 0.1),
        Detection::new( 0.7, 0.7, 0.1, 0.1)
    ];

    let out1 = tracker.update(0, "2025-01-01", &dets);
    assert_eq!(out1.tracked_objects.len(), 2);

    let dets2 = vec![
        Detection::new(0.3, 0.3, 0.1, 0.1),
        Detection::new(0.71, 0.71, 0.1, 0.1)
    ];
    let out2 = tracker.update(1, "2025-01-01", &dets2);
    assert_eq!(out2.tracked_objects[0].id, out1.tracked_objects[0].id);
    assert_eq!(out2.tracked_objects[1].id, out1.tracked_objects[1].id);
}

