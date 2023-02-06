use gstreamer::prelude::*;


fn tutorial_main() {
    // Initialize GStreamer
    gstreamer::init().unwrap();

    // Build the pipeline
    let path =
        "./traffic_video.mp4";
    let pipeline = gstreamer::parse_launch(&format!("filesrc location={path} ! decodebin ! xvimagesink")).unwrap();

    // Start playing
    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    tutorial_main();
}