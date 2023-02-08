use gstreamer::prelude::*;


fn tutorial_main() {
    // Initialize GStreamer
    gstreamer::init().unwrap();

    // Build the pipeline
    let path = "./traffic_video.mp4";

    let filesrc = gstreamer::ElementFactory::make("filesrc")
        .name("filesrc")
        .property_from_str("location", path)
        .build()
        .expect("Could not create filesrc");
        
    let qtdemux = gstreamer::ElementFactory::make("qtdemux")
        .name("qtdemux")
        .build()
        .expect("Could not create qtdemux");
        
    let h264parse = gstreamer::ElementFactory::make("h264parse")
        .name("h264parse")
        .build()
        .expect("Could not create h264parse");
        
    let avdec_h264 = gstreamer::ElementFactory::make("avdec_h264")
        .name("avdec_h264")
        .build()
        .expect("Could not create avdec_h264");

    let xvimagesink = gstreamer::ElementFactory::make("xvimagesink")
        .name("xvimagesink")
        .build()
        .expect("Could not make xvimagesink");

    let pipeline = gstreamer::Pipeline::builder().name("test-pipeline").build();

    pipeline.add_many(&[&filesrc, &qtdemux, &h264parse, &avdec_h264, &xvimagesink]).unwrap();
    filesrc.link(&qtdemux).expect("Error linking filesrc to qtdemux");
    // qtdemux.link(&h264parse).expect("Error linking qtdemux to h264parse");
    h264parse.link(&avdec_h264).expect("Error linking h264parse to avdec_h264");
    avdec_h264.link(&xvimagesink).expect("Error linking avdec_h264 to xvimagesink");

    qtdemux.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());

        let sink_pad = h264parse.static_pad("sink").expect("Failed to get static sink pad from h264parse");
        if sink_pad.is_linked() {
            println!("We are already linked. Ignoring...");
            return;
        }

        let new_pad_caps = src_pad.current_caps().expect("Failed to get caps of new pad");
        let new_pad_struct = new_pad_caps.structure(0).expect("Failed to get first structure of caps");
        let new_pad_type = new_pad_struct.name();

        let is_video = new_pad_type.starts_with("video");
        if is_video {
            println!("Found first video pad {}. Trying to link.", new_pad_type);
            let res = src_pad.link(&sink_pad);
            match res {
                Ok(_) => println!("Link succeeded."),
                Err(_) => println!("Link failed."),
            }
        }
    });
    
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