use crate::detection::Detection;

#[derive(Debug, Clone)]
pub struct Track {
    id: u32,
    bbox: (f32, f32, f32, f32), // x1, y1, x2, y2
    last_seen: u32,
    missing: u32,
    history: Vec<(f32, f32)>,
}

impl Track {
    pub fn new(id: u32, det: &Detection, frame_id: u32) -> Self {
        let bbox = det.bbox();
        Self {
            id,
            bbox,
            last_seen: frame_id,
            missing: 0,
            history: vec![det.center()],
        }
    }

    pub fn update(&mut self, det: &Detection, frame_id: u32) {
        self.bbox = det.bbox();
        self.last_seen = frame_id;
        self.missing = 0;
        self.history.push(det.center());
    }

    pub fn mark_missing(&mut self) {
        self.missing += 1;
    }

    pub fn iou(&self, det: &Detection) -> f32 {
        let (x1, y1, x2, y2) = self.bbox;
        let (dx1, dy1, dx2, dy2) = det.bbox();

        let inter_x1 = x1.max(dx1);
        let inter_y1 = y1.max(dy1);
        let inter_x2 = x2.min(dx2);
        let inter_y2 = y2.min(dy2);

        let inter_area = (inter_x2 - inter_x1).max(0.0) * (inter_y2 - inter_y1).max(0.0);
        let area_self = (x2 - x1) * (y2 - y1);
        let area_det = (dx2 - dx1) * (dy2 - dy1);

        inter_area / (area_self + area_det - inter_area + 1e-6)
    }

    pub fn to_output(&self) -> crate::tracker::TrackedObject {
        let (x1, y1, x2, y2) = self.bbox;
        crate::tracker::TrackedObject {
            id: self.id,
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn last_seen_frame(&self) -> u32 {
        self.last_seen
    }

    pub fn is_missing_too_long(&self, max: u32) -> bool {
        self.missing > max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detection::Detection;

    fn sample_detection(x: f32, y: f32) -> Detection {
        Detection::new(x, y, 0.1, 0.1)
    }

    #[test]
    fn test_iou_same_box() {
        let det = sample_detection(0.5, 0.5);
        let track = Track::new(1, &det, 0);
        assert!((track.iou(&det) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_iou_partial_overlap() {
        let d1 = sample_detection(0.5, 0.5);
        let d2 = sample_detection(0.55, 0.5);
        let track = Track::new(1, &d1, 0);
        let iou = track.iou(&d2);
        assert!(iou > 0.0 && iou < 1.0);
    }

    #[test]
    fn test_missing_logic() {
        let det = sample_detection(0.5, 0.5);
        let mut track = Track::new(1, &det, 0);
        track.mark_missing();
        track.mark_missing();
        assert_eq!(track.is_missing_too_long(1), true);
        assert_eq!(track.is_missing_too_long(2), false);
    }
}
