to gen video, run video script then ffmpeg

`ffmpeg -framerate 30 -i tmp/image_%03d.png -c:v libx264 -pix_fmt yuv420p output.mp4`
