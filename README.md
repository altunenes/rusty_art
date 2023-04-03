# rusty_art


![227796857-e73c8e66-1446-4600-8018-aeaa6a59a4a4](https://user-images.githubusercontent.com/54986652/227951137-35ab864e-3329-4ef0-a4aa-2347f07296ca.png)

Creative coding with Rust.

To run the scripts, in the root directory of the project, type the commands in the following style.

    ```bash
        cargo run --release --bin <scriptname>
    ```

If you want to create high resolution videos, add the following code at the end of the script:

    ```rust
        if app.elapsed_frames() % 1 == 0 {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:1}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
    draw.to_frame(app, &frame).unwrap();
    ```

Copy the outout images to a folder and run the following command in the folder.
    
    ```bash
        ffmpeg -r 60 -f image2 -i %d.png -vcodec libx264 -crf 25 -pix_fmt yuv420p output.mp4
    ```


Play with the math and see significant changes in the output.

Some Examples:


https://user-images.githubusercontent.com/54986652/229513354-4b6652a5-3bef-4c99-9fed-22f35d3ea71f.mp4



https://user-images.githubusercontent.com/54986652/229513423-179042a9-4594-4bd6-983b-74363446e9ae.mp4


https://user-images.githubusercontent.com/54986652/229513544-d1a610ff-c6ec-4476-9dd4-6e006d9de5de.mp4



https://user-images.githubusercontent.com/54986652/229513630-592b233d-7773-4cd8-910a-264b45c2d447.mp4


https://user-images.githubusercontent.com/54986652/229513748-ffa95d44-a6df-4b6a-9265-fbbe1f68eb99.mp4




https://user-images.githubusercontent.com/54986652/229514126-6c342946-c162-4839-a5cf-531bc11fdea9.mp4




https://user-images.githubusercontent.com/54986652/229513824-85ea53f6-d971-474a-8566-b454b8156b42.mp4





