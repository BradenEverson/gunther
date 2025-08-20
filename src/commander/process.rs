//! CV Implementation

use super::{Commander, op::Op};
use core::f32;

use ndarray::Array4;
use opencv::{
    core::Mat,
    highgui,
    imgproc::{self, LINE_8},
    prelude::*,
    videoio,
};
use ort::session::Session;

const SIZE: i32 = 320;

fn translate_to_new(val: f32, old: i32, new: i32) -> f32 {
    (val / old as f32) * new as f32
}

impl Commander {
    /// Entire commander's process
    pub fn process(&self) {
        highgui::named_window("window", highgui::WINDOW_FULLSCREEN)
            .expect("Oh no couldn't make a window");

        let mut cam =
            videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Oh no couldn't find a camera");

        cam.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)
            .expect("Couldn't set width");
        cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)
            .expect("Couldn't set height");

        let mut unflipped = Mat::default();
        let mut frame = Mat::default();

        let mut model = Session::builder()
            .expect("Bruh couldn't even start a session")
            .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
            .expect("Couldn't set optimization")
            .with_intra_threads(4)
            .expect("No threads :(")
            .commit_from_file("yolov8n-pose-320.onnx")
            .expect("No model???");

        cam.read(&mut frame).expect("Oh no camera got a little shy");
        let size = frame.size().expect("No size???");

        let width = size.width;
        let height = size.height;

        loop {
            cam.read(&mut unflipped)
                .expect("Oh no camera got a little shy");

            opencv::core::flip(&unflipped, &mut frame, 0).expect("Failed to flip");

            let input_tensor = preprocess_frame(&frame).expect("Couldn't preprocess");
            let value = ort::value::Value::from_array(input_tensor).expect("No value :(");

            let outputs = model.run(ort::inputs!["images" => value]).unwrap();
            let predictions = outputs["output0"]
                .try_extract_array::<f32>()
                .expect("No predictions?");

            let shape = predictions.shape();
            let num_detections = shape[2];

            let mut detections = vec![];

            for idx in 0..num_detections {
                let confidence = predictions[[0, 4, idx]];

                if confidence > 0.5 {
                    let mut keypoints = Vec::new();
                    for kp_idx in 0..17 {
                        let offset = 5 + kp_idx * 3;
                        let x = predictions[[0, offset, idx]];
                        let y = predictions[[0, offset + 1, idx]];

                        let x = translate_to_new(x, SIZE, width);
                        let y = translate_to_new(y, SIZE, height);

                        let kp_conf = predictions[[0, offset + 2, idx]];

                        if kp_conf > 0.2 {
                            keypoints.push(KeyPoint { x, y });
                        } else {
                            keypoints.push(KeyPoint {
                                x: f32::NAN,
                                y: f32::NAN,
                            });
                        }
                    }

                    detections.push(PoseDetection { keypoints });
                }
            }

            if detections.len() > 0 {
                let detection = &detections[0];
                for kp in &detection.keypoints {
                    imgproc::circle(
                        &mut frame,
                        opencv::core::Point::new(kp.x as i32, kp.y as i32),
                        1,
                        opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                        2,
                        LINE_8,
                        0,
                    )
                    .expect("circle");
                }
            }

            highgui::imshow("window", &frame).expect("Oh no couldn't show image to GUI");

            let key =
                highgui::wait_key(1).expect("Oh no couldn't... wait what I couldn't grab a key?");
            if key == 113 {
                break;
            }
        }
    }
}

fn preprocess_frame(frame: &Mat) -> opencv::Result<Array4<f32>> {
    let mut resized = Mat::default();
    opencv::imgproc::resize(
        frame,
        &mut resized,
        opencv::core::Size::new(SIZE, SIZE),
        0.0,
        0.0,
        opencv::imgproc::INTER_LINEAR,
    )?;

    let mut rgb = Mat::default();
    opencv::imgproc::cvt_color(&resized, &mut rgb, opencv::imgproc::COLOR_BGR2RGB, 0)?;

    let size = SIZE as usize;
    let mut array = Array4::<f32>::zeros((1, 3, size, size));

    for y in 0..SIZE {
        for x in 0..SIZE {
            let pixel = rgb.at_2d::<opencv::core::Vec3b>(y, x)?;
            array[[0, 0, y as usize, x as usize]] = pixel[2] as f32 / 255.0; // R
            array[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0; // G
            array[[0, 2, y as usize, x as usize]] = pixel[0] as f32 / 255.0; // B
        }
    }

    Ok(array)
}

#[derive(Debug)]
struct PoseDetection {
    keypoints: Vec<KeyPoint>,
}

#[derive(Debug)]
struct KeyPoint {
    x: f32,
    y: f32,
}
