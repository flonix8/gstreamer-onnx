# gstreamer-onnx

## Successful GStreamer pipelines
- Play supplied .mp4 file
  `gst-launch-1.0 filesrc location=traffic_video.mp4 ! decodebin ! xvimagesink`
  `gst-launch-1.0 filesrc location=traffic_video.mp4 ! qtdemux ! h264parse ! avdec_h264 ! xvimagesink`

- Play webcam
  `gst-launch-1.0 v4l2src ! videoconvert ! xvimagesink`