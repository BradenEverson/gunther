//! YOLO Pose Convenience Structs

use opencv::imgproc::LINE_8;

/// Nose Keypoint Index
pub const NOSE: usize = 0;
/// Left Eye Keypoint Index
pub const LEFT_EYE: usize = 1;
/// Right Eye Keypoint Index
pub const RIGHT_EYE: usize = 2;
/// left ear Keypoint Index
pub const LEFT_EAR: usize = 3;
/// Right Ear Keypoint Index
pub const RIGHT_EAR: usize = 4;
/// Left Shoulder Keypoint Index
pub const LEFT_SHOULDER: usize = 5;
/// Right Shoulder Keypoint Index
pub const RIGHT_SHOULDER: usize = 6;
/// Left Elbow Keypoint Index
pub const LEFT_ELBOW: usize = 7;
/// Right Elbow Keypoint Index
pub const RIGHT_ELBOW: usize = 8;
/// Left Wrist Keypoint Index
pub const LEFT_WRIST: usize = 9;
/// Right Wrist Keypoint Index
pub const RIGHT_WRIST: usize = 10;
/// Left Hip Keypoint Index
pub const LEFT_HIP: usize = 11;
/// Right Hip Keypoint Index
pub const RIGHT_HIP: usize = 12;
/// Left Knee Keypoint Index
pub const LEFT_KNEE: usize = 13;
/// Right Knee Keypoint Index
pub const RIGHT_KNEE: usize = 14;
/// Left Ankle Keypoint Index
pub const LEFT_ANKLE: usize = 15;
/// Right Ankle Keypoint Index
pub const RIGHT_ANKLE: usize = 16;

/// A collection of body keypoints
#[derive(Debug, Clone)]
pub struct PoseDetection {
    /// The 17 pose keypoints
    pub keypoints: [Option<KeyPoint>; 17],
}

impl PoseDetection {
    /// Attempts to grab several points, failing if even one isn't found
    pub fn grab_many(&self, requested: &[usize]) -> Option<Vec<KeyPoint>> {
        let mut res = vec![];

        for id in requested {
            res.push(self.keypoints[*id]?);
        }

        Some(res)
    }

    /// If the pose contains hands over their head, we should stop shooting out of the kindness of
    /// our hearts
    pub fn give_mercy(&self) -> bool {
        if let Some(hands_nose) = self.grab_many(&[LEFT_WRIST, RIGHT_WRIST, NOSE]) {
            let l_wrist = hands_nose[0];
            let r_wrist = hands_nose[1];
            let nose = hands_nose[2];

            l_wrist.y > nose.y && r_wrist.y > nose.y
        } else {
            false
        }
    }

    /// Get a point between all valid points
    pub fn avg_all(&self) -> Option<KeyPoint> {
        self.get_avg(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
    }

    /// Get the average of many keypoints, ignoring any that can't be found
    pub fn get_avg(&self, requested: &[usize]) -> Option<KeyPoint> {
        let mut seen = 0;
        let mut kp = KeyPoint::default();

        for req in requested {
            if let Some(k) = self.keypoints[*req] {
                kp.x += k.x;
                kp.y += k.y;
                seen += 1;
            }
        }

        if seen == 0 {
            None
        } else {
            kp.x /= seen as f32;
            kp.y /= seen as f32;
            Some(kp)
        }
    }

    /// Attempt to draw all pose parts we have :)
    pub fn draw_body(&self, frame: &mut opencv::core::Mat) {
        self.draw_line(frame, LEFT_SHOULDER, RIGHT_SHOULDER);

        self.draw_line(frame, LEFT_SHOULDER, LEFT_ELBOW);
        self.draw_line(frame, LEFT_ELBOW, LEFT_WRIST);

        self.draw_line(frame, RIGHT_SHOULDER, RIGHT_ELBOW);
        self.draw_line(frame, RIGHT_ELBOW, RIGHT_WRIST);

        self.draw_line(frame, RIGHT_SHOULDER, RIGHT_HIP);
        self.draw_line(frame, LEFT_SHOULDER, LEFT_HIP);

        self.draw_line(frame, LEFT_HIP, RIGHT_HIP);

        self.draw_line(frame, LEFT_HIP, LEFT_KNEE);
        self.draw_line(frame, RIGHT_HIP, RIGHT_KNEE);

        self.draw_line(frame, LEFT_KNEE, LEFT_ANKLE);
        self.draw_line(frame, RIGHT_KNEE, RIGHT_ANKLE);

        self.draw_line(frame, LEFT_SHOULDER, LEFT_EAR);
        self.draw_line(frame, RIGHT_SHOULDER, RIGHT_EAR);

        self.draw_line(frame, LEFT_EAR, LEFT_EYE);
        self.draw_line(frame, RIGHT_EAR, RIGHT_EYE);

        self.draw_line(frame, LEFT_EYE, NOSE);
        self.draw_line(frame, RIGHT_EYE, NOSE);
    }

    /// Attempt to draw a line
    pub fn draw_line(&self, frame: &mut opencv::core::Mat, from: usize, to: usize) {
        if let Some(line) = self.grab_many(&[from, to]) {
            let shoulder = opencv::core::Point::new(line[0].x as i32, line[0].y as i32);
            let elbow = opencv::core::Point::new(line[1].x as i32, line[1].y as i32);

            opencv::imgproc::line(
                frame,
                shoulder,
                elbow,
                opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                LINE_8,
                0,
            )
            .expect("Line")
        }
    }
}

/// A single keypoint
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyPoint {
    /// X Coordinate
    pub x: f32,
    /// Y Coordinate
    pub y: f32,
}
