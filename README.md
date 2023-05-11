# rusty_art

[![Rust](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml)
[![CodeFactor](https://www.codefactor.io/repository/github/altunenes/rusty_art/badge)](https://www.codefactor.io/repository/github/altunenes/rusty_art)

![227796857-e73c8e66-1446-4600-8018-aeaa6a59a4a4](https://user-images.githubusercontent.com/54986652/227951137-35ab864e-3329-4ef0-a4aa-2347f07296ca.png)

Creative coding with Rust!


In this repository, I will create animations and optical illusions that allow for the generation of various patterns in my spare time. So, this repo is consistently updated and current :=)

Nearly all of my scripts feature a graphical user interface (GUI), enabling real-time adjustments to the animations and the creation of thousands of unique patterns.

I hope you enjoy it!


## Usage/Installation:

*This section is intended for those who are new to GitHub or Rust and may not be familiar with these tools.*

1- Install [Rust Programming Language](https://www.rust-lang.org/tools/install)

2- click on the green "Code" button at the top right of this page. Then, select "Download ZIP" from the dropdown menu. After the ZIP file finishes downloading, extract it to a folder on your computer

3- Open the terminal (also known as command prompt or cmd) on your computer. You can simply open it in the folder "rusty_spelling" by right-clicking on the folder and selecting "Open in Terminal".


To run the scripts, in the root directory of the project, type the commands in the following style.

    ```bash
        cargo run --release --bin <scriptname>
    ```

That's it! Feel free to contact me if you have trouble opening/running scripts.


### Saving Frames

To create high-resolution videos, you can save each frame as a PNG image by holding down the `spacebar` while the script is running. This will save each frame in a folder called "frames" which will be created automatically in your current directory.


Once you've saved all the frames you want, you can create a video file by copying the images to a folder and running the following command in that folder:

    
    ```bash
        ffmpeg -r 60 -f image2 -i %d.png -vcodec libx264 -crf 25 -pix_fmt yuv420p output.mp4
    ```

This command will use the images in the "frames" folder to create a video file named "output.mp4" in the same folder. The video will be encoded with the libx264 codec at a constant rate factor of 25 and with a pixel format of yuv420p. The frame rate will be set to 60 frames per second (-r 60).

note: you have to install [ffmpeg](https://ffmpeg.org/) to save frames.


Some Examples:

attractors:

https://user-images.githubusercontent.com/54986652/231419418-78d3abe9-bb9d-4a5e-83a6-86eacb8a1e79.mp4


neural network sim :)


https://user-images.githubusercontent.com/54986652/236688398-59a39a24-db31-4bc0-9fbb-76ff8d58a7cb.mp4


sometimes simplicity is always best :)

https://user-images.githubusercontent.com/54986652/236439899-43570ee1-0093-4aee-b38b-49a46b59099e.mp4


pixel rain

https://user-images.githubusercontent.com/54986652/235495648-8c279bd8-2606-4dc9-a3ab-1c266e1ffbcf.mp4


peace :)

https://user-images.githubusercontent.com/54986652/234987806-603716b4-a3e7-4578-905f-ffe99c8a124b.mp4

famous "cafewall illusion"

https://user-images.githubusercontent.com/54986652/232924117-17765b32-5da4-4c57-88d5-cdc9eecc7ff4.mp4

Fourier cycloids for encoding Perlin noise or sin waves

https://user-images.githubusercontent.com/54986652/234093920-190133d0-f60c-40f5-87a2-6eead393e50c.mp4

Pink diamond doesn't move! Reproduction [of this article ](https://journals.sagepub.com/doi/full/10.1177/2041669518815708)


https://user-images.githubusercontent.com/54986652/233513052-2833af0d-df4c-4793-910a-50bd5b6f19ba.mp4


Ferris <3 in sine wave oscillations!!



https://github.com/altunenes/rusty_art/assets/54986652/f4c07297-aaa5-4df3-859f-354a6a898754



Sine-Waves From night to morning :)

https://user-images.githubusercontent.com/54986652/232167815-ecb21c06-210e-4f54-8d45-942af43d0acb.mp4

Munker illusion! The colors of the left and right circles are identical.

https://user-images.githubusercontent.com/54986652/233230706-3cec1c65-af60-4a39-8290-86c8d92d1cbb.mp4

Liquid Lisa :)

https://user-images.githubusercontent.com/54986652/232653308-5b19b38b-b33d-40f3-908a-9635dff92b43.mp4

How to brain process faces: From coarse to fine integration!

https://user-images.githubusercontent.com/54986652/235327714-f4e5bc0c-0074-42d3-9cc1-82395c4d561f.mp4

Draw not a perfect circle with triangles :)

https://user-images.githubusercontent.com/54986652/230115569-f7ad4bb6-0bef-4f4b-8952-439be7a2a64e.mp4

The Night Watch got caught in a pixel rain! 

https://user-images.githubusercontent.com/54986652/235557669-d9d6f605-4939-401a-8a9f-5995f69002d3.mp4

psychodelic experience :)))

https://user-images.githubusercontent.com/54986652/234985596-5d97bfbb-98d7-40a2-95bf-8b8c3a5b46ef.mp4

"static" attractors!

https://user-images.githubusercontent.com/54986652/231308988-04f1cdae-27b8-4fd1-a84c-e69b06bf6b1b.mp4

golden ratio!

https://user-images.githubusercontent.com/54986652/229513630-592b233d-7773-4cd8-910a-264b45c2d447.mp4

perlin noise + sine waves

https://user-images.githubusercontent.com/54986652/232155899-f0a1d352-0efd-4c60-9e35-2cc65bbe5c1c.mp4

excited polylines :)

https://user-images.githubusercontent.com/54986652/234881771-47a903ca-0888-42a1-9879-2389c962adb3.mp4

pinna illusion!

https://user-images.githubusercontent.com/54986652/236040873-5c9582ee-fe01-4e28-9240-155065f687a2.mp4

Enigma

https://user-images.githubusercontent.com/54986652/230368034-813a0bc0-06f5-4410-945c-df72f44c5932.mp4

stop at 01:00 Can you make it look like anything? 
PS: anatomy ;=)

https://user-images.githubusercontent.com/54986652/234882644-5b214205-3de5-47ce-8907-ba60d62e4a83.mp4

a demo for lilac chaster illusion. If you focus your eyes properly on the center, you will perceive a vivid green hue once the violet area disappears.

https://user-images.githubusercontent.com/54986652/233791613-887a99ed-c3e8-4a20-8b85-0514dfdd6f56.mp4

Flower :))

https://user-images.githubusercontent.com/54986652/235549644-9d76292a-785c-44e5-9dd5-2b1c175a49f0.mp4


brain gathering the signals 

https://user-images.githubusercontent.com/54986652/236209923-6a764d4c-ff97-4670-941f-07b1c0839cbd.mp4









