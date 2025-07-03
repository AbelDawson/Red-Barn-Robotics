use crate::detection::Detection;
use crate::track::Track;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TrackedObject {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize)]
pub struct FrameOutput {
    pub frame_id: u32,
    pub timestamp: String,
    pub tracked_objects: Vec<TrackedObject>,
}

pub struct Tracker {
    tracks: Vec<Track>,
    next_id: u32,
    max_missing: u32,
    iou_threshold: f32,
}

impl Tracker {
    pub fn new(max_missing: u32, iou_threshold: f32) -> Self {
        Self {
            tracks: vec![],
            next_id: 1,
            max_missing,
            iou_threshold,
        }
    }

    pub fn update(
        &mut self,
        frame_id: u32,
        timestamp: &str,
        detections: &[Detection],
    ) -> FrameOutput {
        let mut assigned = vec![false; detections.len()];

        for track in self.tracks.iter_mut() {
            let mut best_iou = 0.0;
            let mut best_idx = None;

            for (i, det) in detections.iter().enumerate() {
                if assigned[i] {
                    continue;
                }
                let iou = track.iou(det);
                if iou > self.iou_threshold && iou > best_iou {
                    best_iou = iou;
                    best_idx = Some(i);
                }
            }


            if let Some(idx) = best_idx {
                track.update(&detections[idx], frame_id);
                assigned[idx] = true;
            } else {
                track.mark_missing();
            }
        }

        // Add unmatched detections as new tracks
        for (i, det) in detections.iter().enumerate() {
            if !assigned[i] {
                let new_track = Track::new(self.next_id, det, frame_id);
                self.tracks.push(new_track);
                self.next_id += 1;
            }
        }

        // Remove old tracks
        self.tracks.retain(|t| !t.is_missing_too_long(self.max_missing));

        let tracked_objects = self
            .tracks
            .iter()
            .filter(|t| t.last_seen_frame() == frame_id)
            .map(|t| t.to_output())
            .collect();

        FrameOutput {
            frame_id,
            timestamp: timestamp.to_string(),
            tracked_objects,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detection::Detection;

    fn det(x: f32, y: f32) -> Detection {
        Detection::new( x, y, 0.1, 0.1 )
    }

    #[test]
    fn test_create_new_track() {
        let mut tracker = Tracker::new(3, 0.3);
        let dets = vec![det(0.1, 0.1)];
        let frame = tracker.update(0, "ts", &dets);
        assert_eq!(frame.tracked_objects.len(), 1);
        assert_eq!(frame.tracked_objects[0].id, 1);
    }

    #[test]
    fn test_track_persistence() {
        let mut tracker = Tracker::new(3, 0.3);
        let dets = vec![det(0.1, 0.1)];
        let f1 = tracker.update(0, "ts", &dets);
        let f2 = tracker.update(1, "ts", &[det(0.1, 0.1)]);
        assert_eq!(f2.tracked_objects[0].id, f1.tracked_objects[0].id);
    }

    #[test]
    fn test_track_disappearance_and_reassignment() {
        let mut tracker = Tracker::new(2, 0.3);
        tracker.update(0, "ts", &[det(0.1, 0.1)]);
        tracker.update(1, "ts", &[]);
        tracker.update(2, "ts", &[]);
        tracker.update(2, "ts", &[]);
        tracker.update(3, "ts", &[det(0.1, 0.1)]);
        let output = tracker.update(4, "ts", &[det(0.1, 0.1)]);
        assert_eq!(output.tracked_objects[0].id, 2); // new ID after deletion
    }
}

