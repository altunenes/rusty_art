# rusty_art

[![Rust](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml)


![227796857-e73c8e66-1446-4600-8018-aeaa6a59a4a4](https://user-images.githubusercontent.com/54986652/227951137-35ab864e-3329-4ef0-a4aa-2347f07296ca.png)

Creative coding with Rust.

## Usage/Installation:

*This section is intended for those who are new to GitHub or Rust and may not be familiar with these tools.*

1- To get started, you'll need to install Rust on your computer. Rust is a programming language that Rusty Art uses to generate art. You can download Rust from this website: [here](https://www.rust-lang.org/tools/install). Follow the installation instructions for your operating system.

2- Once you have Rust installed, click on the green "Code" button at the top right of this page. Then, select "Download ZIP" from the dropdown menu. After the ZIP file finishes downloading, extract it to a folder on your computer

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


Some Examples:


https://user-images.githubusercontent.com/54986652/231419418-78d3abe9-bb9d-4a5e-83a6-86eacb8a1e79.mp4



https://user-images.githubusercontent.com/54986652/235495648-8c279bd8-2606-4dc9-a3ab-1c266e1ffbcf.mp4



https://user-images.githubusercontent.com/54986652/234987806-603716b4-a3e7-4578-905f-ffe99c8a124b.mp4


https://user-images.githubusercontent.com/54986652/232924117-17765b32-5da4-4c57-88d5-cdc9eecc7ff4.mp4

https://user-images.githubusercontent.com/54986652/234093920-190133d0-f60c-40f5-87a2-6eead393e50c.mp4

nothing moves!


https://user-images.githubusercontent.com/54986652/233513052-2833af0d-df4c-4793-910a-50bd5b6f19ba.mp4


https://user-images.githubusercontent.com/54986652/232167815-ecb21c06-210e-4f54-8d45-942af43d0acb.mp4


https://user-images.githubusercontent.com/54986652/233230706-3cec1c65-af60-4a39-8290-86c8d92d1cbb.mp4


https://user-images.githubusercontent.com/54986652/232653308-5b19b38b-b33d-40f3-908a-9635dff92b43.mp4


https://user-images.githubusercontent.com/54986652/235327714-f4e5bc0c-0074-42d3-9cc1-82395c4d561f.mp4



https://user-images.githubusercontent.com/54986652/230115569-f7ad4bb6-0bef-4f4b-8952-439be7a2a64e.mp4



https://user-images.githubusercontent.com/54986652/234985596-5d97bfbb-98d7-40a2-95bf-8b8c3a5b46ef.mp4


https://user-images.githubusercontent.com/54986652/229513354-4b6652a5-3bef-4c99-9fed-22f35d3ea71f.mp4

https://user-images.githubusercontent.com/54986652/231308988-04f1cdae-27b8-4fd1-a84c-e69b06bf6b1b.mp4


https://user-images.githubusercontent.com/54986652/229513423-179042a9-4594-4bd6-983b-74363446e9ae.mp4


https://user-images.githubusercontent.com/54986652/229513544-d1a610ff-c6ec-4476-9dd4-6e006d9de5de.mp4



https://user-images.githubusercontent.com/54986652/229513630-592b233d-7773-4cd8-910a-264b45c2d447.mp4


https://user-images.githubusercontent.com/54986652/232155899-f0a1d352-0efd-4c60-9e35-2cc65bbe5c1c.mp4



https://user-images.githubusercontent.com/54986652/229513748-ffa95d44-a6df-4b6a-9265-fbbe1f68eb99.mp4


https://user-images.githubusercontent.com/54986652/232165755-5e67a7b2-613f-4800-82d4-454160df0cc1.mp4



https://user-images.githubusercontent.com/54986652/234881771-47a903ca-0888-42a1-9879-2389c962adb3.mp4



https://user-images.githubusercontent.com/54986652/230368034-813a0bc0-06f5-4410-945c-df72f44c5932.mp4


https://user-images.githubusercontent.com/54986652/234882644-5b214205-3de5-47ce-8907-ba60d62e4a83.mp4


https://user-images.githubusercontent.com/54986652/233791613-887a99ed-c3e8-4a20-8b85-0514dfdd6f56.mp4


https://user-images.githubusercontent.com/54986652/222890920-21105bc5-5fc7-4eb2-97a0-b953a456b00d.mp4


this one is optical illusion :) (blue and yellow rects have the const speed)

https://user-images.githubusercontent.com/54986652/222891008-f82119e1-3abf-4c8d-953b-8a44e6d55692.mp4


This one too: (it's 2d but you may percevie at as a 3d!)

https://user-images.githubusercontent.com/54986652/230370841-4b617154-025c-4501-ab40-341bf123cee7.mp4

bright illusion demo!
![glow](https://user-images.githubusercontent.com/54986652/233481553-5dd564bb-4930-473b-b374-a7227ef16698.png)


https://user-images.githubusercontent.com/54986652/229129096-49aa97a3-24a2-49c3-a9f0-5cd40cfcb779.mp4





