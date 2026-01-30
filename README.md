# Rust Raytracer / Raymarcher

## Generating Video

To generate video, run video script in examples then ffmpeg:

`ffmpeg -framerate 30 -i tmp/image_%03d.png -c:v libx264 -pix_fmt yuv420p output.mp4`

## Raymarching materials

To apply materials to a raymarched object, we need some extra data along with the SDF.
This should consist of a "material field" which assigns the matterial to eeach point in space with respect to our SDF.
For a simple union of shapes, we can take the shape to which our point is closest, and take the material from there.

## Materials List

- Diffuse coloured object (needs colour)
- Specular object (mirror)
- Glossy object (diffuse mirror)
- Glass (reflect, refract, absorb)
- (Volumetric) (fog)
- (Light)
