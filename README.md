# rusty_art

[![Rust](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml)


![227796857-e73c8e66-1446-4600-8018-aeaa6a59a4a4](https://user-images.githubusercontent.com/54986652/227951137-35ab864e-3329-4ef0-a4aa-2347f07296ca.png)

Creative coding with Rust.

To run the scripts, in the root directory of the project, type the commands in the following style.

    ```bash
        cargo run --release --bin <scriptname>
    ```

If you want to create high resolution videos hold the `spacebar` while the script is running. This will save the each frame as a png image in your current directory (saves frames as long as you hold it down, if you release it, it will stop saving frames)


Copy the images to a folder and run the following command in the folder:
    
    ```bash
        ffmpeg -r 60 -f image2 -i %d.png -vcodec libx264 -crf 25 -pix_fmt yuv420p output.mp4
    ```
this will create a video file named `output.mp4` in the current directory.

Play with the math and see significant changes in the output. 

Update: 
From now on, I will be adding GUI to the scripts. So, you can play with the math and see the changes in real time without having to recompile the code.

This is my first attempt at GUI (for triangles.rs):


Some Examples:


https://user-images.githubusercontent.com/54986652/230115569-f7ad4bb6-0bef-4f4b-8952-439be7a2a64e.mp4



https://user-images.githubusercontent.com/54986652/229513354-4b6652a5-3bef-4c99-9fed-22f35d3ea71f.mp4



https://user-images.githubusercontent.com/54986652/229513423-179042a9-4594-4bd6-983b-74363446e9ae.mp4


https://user-images.githubusercontent.com/54986652/229513544-d1a610ff-c6ec-4476-9dd4-6e006d9de5de.mp4



https://user-images.githubusercontent.com/54986652/229513630-592b233d-7773-4cd8-910a-264b45c2d447.mp4


https://user-images.githubusercontent.com/54986652/229513748-ffa95d44-a6df-4b6a-9265-fbbe1f68eb99.mp4


https://user-images.githubusercontent.com/54986652/230368034-813a0bc0-06f5-4410-945c-df72f44c5932.mp4



https://user-images.githubusercontent.com/54986652/229513824-85ea53f6-d971-474a-8566-b454b8156b42.mp4


https://user-images.githubusercontent.com/54986652/222890920-21105bc5-5fc7-4eb2-97a0-b953a456b00d.mp4


this one is optical illusion :) (blue and yellow rects have the const speed)

https://user-images.githubusercontent.com/54986652/222891008-f82119e1-3abf-4c8d-953b-8a44e6d55692.mp4


This one too: (it's 2d but you may percevie at as a 3d!)

https://user-images.githubusercontent.com/54986652/230370841-4b617154-025c-4501-ab40-341bf123cee7.mp4




https://user-images.githubusercontent.com/54986652/229129096-49aa97a3-24a2-49c3-a9f0-5cd40cfcb779.mp4
