use opencv::core::{min_max_loc, no_array, Mat, Point2f, VecN, CV_32FC1, Point2i};
use opencv::imgcodecs::{imread, IMREAD_GRAYSCALE};
use opencv::imgproc::{match_template, TM_CCORR};
use opencv::prelude::MatTraitConstManual;

fn main() {
    let img1 = imread("img.png", IMREAD_GRAYSCALE).expect("can't read image");
    let template = imread("template.png", IMREAD_GRAYSCALE).expect("can't read template");
    let size = img1.size().unwrap();
    let mut result = Mat::new_size_with_default(size, CV_32FC1, VecN::new(0.0, 0.0, 0.0, 0.0))
        .expect("can't create mat for result");

    match_template(&img1, &template, &mut result, TM_CCORR, &no_array())
        .expect("can't match template");

    let mut max_loc = Point2i::new(0, 0);
    min_max_loc(&result, None, None, None, Some(&mut max_loc), &no_array())
        .expect("can't get min_max loc");
    println!("max_loc: {:?}", max_loc);
}
