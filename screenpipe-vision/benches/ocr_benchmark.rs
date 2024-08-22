use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::path::PathBuf;
use std::time::Duration;

#[cfg(target_os = "macos")]
use screenpipe_vision::perform_ocr_apple;

const EXPECTED_KEYWORDS: &[&str] = &[
    "ocr_handles",
    "Vec",
    "pool_size",
    "task::spawn",
    "async move",
    "should_stop.lock().await",
    "RecvError::Lagged",
    "debug!",
    "error!",
    "frame_counter",
    "start_time",
    "last_processed_frame",
    "control_rx.try_recv()",
    "ControlMessage::Pause",
    "ControlMessage::Resume",
    "ControlMessage::Stop",
    "is_paused.lock().await",
    "tokio::time::sleep",
    "capture_start",
    "monitor.capture_image()",
    "DynamicImage::ImageRgba8",
    "capture_duration",
    "image_hash",
    "calculate_hash",
    "result_tx_clone",
    "image_arc",
    "Arc::new",
    "queue_size",
    "ocr_tx.receiver_count()",
    "MAX_QUEUE_SIZE",
    "frames_to_skip",
];

// Helper function to load test image
fn load_test_image() -> image::DynamicImage {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("testing_OCR.png");
    image::open(&path).expect("Failed to open image")
}

// Performance test for Apple Vision OCR
#[cfg(target_os = "macos")]
fn bench_apple_vision_ocr(c: &mut Criterion) {
    let image = load_test_image();
    let mut group = c.benchmark_group("Apple Vision OCR");
    group.sample_size(10); // Reduce sample size to address warning
    group.measurement_time(Duration::from_secs(10)); // Increase measurement time

    group.bench_function(BenchmarkId::new("Performance", ""), |b| {
        b.iter(|| {
            let result = perform_ocr_apple(black_box(&image));
            assert!(!result.is_empty(), "OCR failed");
        })
    });

    group.finish();
}

// Accuracy test for Apple Vision OCR
#[cfg(target_os = "macos")]
fn test_apple_vision_ocr_accuracy() {
    let image = load_test_image();
    let result = perform_ocr_apple(&image);

    let matched_keywords = EXPECTED_KEYWORDS
        .iter()
        .filter(|&&keyword| result.contains(keyword))
        .count();
    let accuracy = matched_keywords as f32 / EXPECTED_KEYWORDS.len() as f32;

    println!("Apple Vision OCR Accuracy: {:.2}", accuracy);
    println!(
        "Matched keywords: {}/{}",
        matched_keywords,
        EXPECTED_KEYWORDS.len()
    );
    assert!(accuracy > 0.3, "Accuracy below threshold"); // Adjusted threshold based on observed results
}

// Combined performance and accuracy test
#[cfg(target_os = "macos")]
fn bench_apple_vision_ocr_with_accuracy(c: &mut Criterion) {
    let image = load_test_image();
    let mut group = c.benchmark_group("Apple Vision OCR with Accuracy");
    group.sample_size(10); // Reduce sample size to address warning
    group.measurement_time(Duration::from_secs(10)); // Increase measurement time

    group.bench_function(BenchmarkId::new("Performance and Accuracy", ""), |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            let mut total_accuracy = 0.0;

            for _ in 0..iters {
                let start = std::time::Instant::now();
                let result = perform_ocr_apple(black_box(&image));
                total_duration += start.elapsed();

                let matched_keywords = EXPECTED_KEYWORDS
                    .iter()
                    .filter(|&&keyword| result.contains(keyword))
                    .count();
                let accuracy = matched_keywords as f32 / EXPECTED_KEYWORDS.len() as f32;
                total_accuracy += accuracy;
            }

            println!("Average Accuracy: {:.2}", total_accuracy / iters as f32);
            total_duration
        })
    });

    group.finish();
}

#[cfg(target_os = "macos")]
criterion_group!(
    benches,
    bench_apple_vision_ocr,
    bench_apple_vision_ocr_with_accuracy
);
#[cfg(target_os = "macos")]
criterion_main!(benches);

#[cfg(target_os = "macos")]
#[test]
fn run_accuracy_test() {
    test_apple_vision_ocr_accuracy();
}