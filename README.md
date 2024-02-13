
#  Rusty_Art 🌈 🎨

[![Rust](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml)

![227796857-e73c8e66-1446-4600-8018-aeaa6a59a4a4](https://user-images.githubusercontent.com/54986652/227951137-35ab864e-3329-4ef0-a4aa-2347f07296ca.png)

**Creative coding with Rust!** 🦀


In this repository, I will create animations and optical illusions that allow for the generation of various patterns in my spare time. So, this repo is consistently updated and current 😄

Most of my scripts feature a **graphical user interface (GUI)**, enabling real-time adjustments to the animations and opening up the possibility to create thousands of unique patterns. 


I hope you enjoy it!


## 🚀 Usage/Installation:

*This section is intended for those who are new to GitHub or Rust and may not be familiar with these tools.*

1- Install [Rust Programming Language](https://www.rust-lang.org/tools/install)

2- Click on the green "Code" button at the top right of this page. Then, select "Download ZIP" from the dropdown menu. After the ZIP file finishes downloading, extract it to a folder on your computer.


3- Open the terminal (also known as command prompt or cmd) on your computer. You can simply open it in the folder "rusty_art" by right-clicking on the folder and selecting "Open in Terminal".


To run the scripts, in the root directory of the project, type the commands in the following style.

    ```bash
        cargo run --release --bin <filename>
    ```

That's it! If you encounter any issues while opening/running scripts, feel free to contact me. 😊


### Saving Frames 📸

To create high-resolution videos, you can save each frame as a PNG image by holding down the <kbd>spacebar</kbd> while the script is running. This will save each frame in a folder called "frames" which will be created automatically in your current directory.


Once you've saved all the frames you want, you can create a video file by copying the images to a folder and running the following command in that folder:

    
    ```bash
        ffmpeg -r 60 -f image2 -start_number 10000 -i %d.png -vf "scale=-2:990,format=yuv420p" -vcodec libx264 -crf 10 output.mp4
    ```

This command will use the images in the "frames" folder to create a video file named "output.mp4" in the same folder. The video will be encoded with the libx264 codec at a constant rate factor of 10 and with a pixel format of yuv420p. The frame rate will be set to 60 frames per second (-r 60).

Note: You need to install [ffmpeg](https://ffmpeg.org/) to create videos.


### Warning ⚠️

I only tested these scripts on my notebook with the following specs:

CPU: Ryzen 7 6800h
Ram: DDR5 16GB
GPU: Nvidia RTX 3060 mobile

And I must say that some of the scripts are very computationally intensive. So, if you have a relatively "low-end" computer, you may not be able to run some of the scripts smoothly. I'm sorry about that. 😔 
And please don't run the scripts on your computer if you don't have a good cooling system. I don't want to be responsible for any damage to your computer. 😅

Always open if you have any suggestions or cheap tricks (in terms of computational complexity) to improve the performance of the scripts. 😊

Scripts that are computationally intensive (in my opinion):
- peace2 and peace3
- mandelbrot
- pixelrain
- attractors
- gabor



#### 🖼️ Some Examples:


| | | |
|:---:|:---:|:---:|
| <video src="https://user-images.githubusercontent.com/54986652/242607093-91bc7605-5223-4eae-a0dc-365e826c0792.mp4" type="video/mp4"></video> **Attractors** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/be9cb26a-912f-4fa3-8172-77c0d03be90d" type="video/mp4"></video> **Neural Network Sim** | <video src="https://user-images.githubusercontent.com/54986652/239741136-4c7041c2-1158-4498-8e23-c5da137eaeda.mp4" type="video/mp4"></video> **Ernst Chladni's Plate Experiments** |
| <video src="https://github.com/altunenes/rusty_art/assets/54986652/17f3037d-e836-46d5-b643-cfde34a48ba4" type="video/mp4"></video> **Simplicity** | <video src="https://user-images.githubusercontent.com/54986652/239285374-8df86f85-7152-4203-aac4-3a9e9e6eca9d.mp4" type="video/mp4"></video> **Gabor** | <video src="https://user-images.githubusercontent.com/54986652/241457493-353cd2b1-c7f9-4369-9226-6d923a278392.mp4" type="video/mp4"></video> **Hilbert** |
| <video src="https://user-images.githubusercontent.com/54986652/235495648-8c279bd8-2606-4dc9-a3ab-1c266e1ffbcf.mp4" type="video/mp4"></video> **Pixel Rain** | <video src="https://user-images.githubusercontent.com/54986652/234987806-603716b4-a3e7-4578-905f-ffe99c8a124b.mp4" type="video/mp4"></video> **Peace2** | <video src="https://user-images.githubusercontent.com/54986652/232924117-17765b32-5da4-4c57-88d5-cdc9eecc7ff4.mp4" type="video/mp4"></video> **Cafewall Illusion** |



| | | |
|:---:|:---:|:---:|
| <video src="https://user-images.githubusercontent.com/54986652/234093920-190133d0-f60c-40f5-87a2-6eead393e50c.mp4" type="video/mp4"></video> **Fourier Cycloids** | <video src="https://user-images.githubusercontent.com/54986652/248949171-4d361b74-e377-4409-9286-525614ff92bf.mp4" type="video/mp4"></video> **Pink Diamond Doesn't Move!** | <video src="https://user-images.githubusercontent.com/54986652/237698718-f4c07297-aaa5-4df3-859f-354a6a898754.mp4" type="video/mp4"></video> **Ferris <3 in Sine Wave Oscillations** |
| <video src="https://github.com/altunenes/rusty_art/assets/54986652/9cf62ec1-558f-4825-9e4a-e03a67936f18" type="video/mp4"></video> **Hilbert Ferris** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/b8d5178f-9155-4b94-8c82-7c3a620d7bb3" type="video/mp4"></video> **Sine-Waves From Night to Morning** | <video src="https://user-images.githubusercontent.com/54986652/233230706-3cec1c65-af60-4a39-8290-86c8d92d1cbb.mp4" type="video/mp4"></video> **Munker Illusion** |
| <video src="https://github.com/altunenes/rusty_art/assets/54986652/f8ccc83c-c864-4b48-aa2b-d0add2eb64db" type="video/mp4"></video> **Leviathan's Enigma Illusion** | <video src="https://user-images.githubusercontent.com/54986652/242370434-9a5e3bb6-b0f8-4b7e-8f25-4cd7f9375088.mp4" type="video/mp4"></video> **Snowflakes!** | <video src="https://user-images.githubusercontent.com/54986652/235327714-f4e5bc0c-0074-42d3-9cc1-82395c4d561f.mp4" type="video/mp4"></video> **How the Brain Processes Faces** |
| <video src="https://github.com/altunenes/rusty_art/assets/54986652/522ca584-4ccf-4755-b655-4d776f0f6001" type="video/mp4"></video> **Draw Not a Perfect Circle with Triangles** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/e9cefb4a-c69b-4cfe-80c3-40d4bd847e4f" type="video/mp4"></video> **The Night Watch in Pixel Rain** | <video src="https://user-images.githubusercontent.com/54986652/234985596-5d97bfbb-98d7-40a2-95bf-8b8c3a5b46ef.mp4" type="video/mp4"></video> **Psychedelic** |


| | | |
|:---:|:---:|:---:|
| <video src="https://github.com/altunenes/rusty_art/assets/54986652/443c6212-b598-42aa-b783-6a729563afee" type="video/mp4"></video> **"Static" Attractors** | <video src="https://user-images.githubusercontent.com/54986652/229513630-592b233d-7773-4cd8-910a-264b45c2d447.mp4" type="video/mp4"></video> **Golden Ratio** | <video src="https://user-images.githubusercontent.com/54986652/248850456-13ddb1f0-3413-4fa1-a436-8008c0e1824a.mp4" type="video/mp4"></video> **Sampling by Ellipses Based on Luminance** |
| <video src="https://user-images.githubusercontent.com/54986652/234881771-47a903ca-0888-42a1-9879-2389c962adb3.mp4" type="video/mp4"></video> **Excited Polylines** | <video src="https://user-images.githubusercontent.com/54986652/236040873-5c9582ee-fe01-4e28-9240-155065f687a2.mp4" type="video/mp4"></video> **Pinna Illusion** | <video src="https://user-images.githubusercontent.com/54986652/242428519-0f9a3dab-061a-4b55-b2c8-ea2757827e20.mp4" type="video/mp4"></video> **Mandelbrot** |
| <video src="https://user-images.githubusercontent.com/54986652/245934580-e42d162b-b071-4e91-949a-de7d2f8dda87.mp4" type="video/mp4"></video> **Sampling Pixels by Luminance** | <video src="https://user-images.githubusercontent.com/54986652/234882644-5b214205-3de5-47ce-8907-ba60d62e4a83.mp4" type="video/mp4"></video> **Anatomy** | <video src="https://user-images.githubusercontent.com/54986652/233791613-887a99ed-c3e8-4a20-8b85-0514dfdd6f56.mp4" type="video/mp4"></video> **Lilac Chaster Illusion** |
| <video src="https://user-images.githubusercontent.com/54986652/235549644-9d76292a-785c-44e5-9dd5-2b1c175a49f0.mp4" type="video/mp4"></video> **Flower** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/131c61f2-f826-4b32-bacd-86baf039dba6" type="video/mp4"></video> **Brain Gathering the Signals** | <video src="https://user-images.githubusercontent.com/54986652/244553268-5714a356-6d3e-43f4-a38e-b976f514eb13.mp4" type="video/mp4"></video> **Attractors2** |

| | | |
|:---:|:---:|:---:|
| <video src="https://user-images.githubusercontent.com/54986652/245289059-0c844c56-bc54-47bb-836c-243a59ceaa67.mp4" type="video/mp4"></video> **Converting Image Pixels into Hypnotic Spiral Line Thickness** | <video src="https://user-images.githubusercontent.com/54986652/249641438-30c75297-b82a-4680-a5f1-0dee8ca0b9bf.mp4" type="video/mp4"></video> **Scramblery** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/1fa3db32-443a-4fb8-928a-54022167a35b" type="video/mp4"></video> **Scramblery2** |
| <video src="https://user-images.githubusercontent.com/54986652/255397004-52a79bc2-d8ed-4549-9ff6-f77ba3bba24a.mp4" type="video/mp4"></video> **Sorting Pixels Based on Luminance** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/7c32f765-5e8f-4162-b401-67c78e82ba18" type="video/mp4"></video> **Ferris with fft!** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/2e4c6cbf-52f7-42c1-8de4-27392835041a" type="video/mp4"></video> **My signature with fft!** |






| | | |
|:---:|:---:|:---:|
| <video src="https://github.com/altunenes/rusty_art/assets/54986652/63f43292-e7cb-4604-984a-06c9934dc50e" type="video/mp4" style="max-width:100px;"></video> **Munker illusion** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/da40be23-8765-4a04-91a5-63b623332a79" type="video/mp4"></video> **Voronoi diagram** |  <video src="https://github.com/altunenes/rusty_art/assets/54986652/d6e88a37-1263-4bc4-9186-eee3eb07ea61" type="video/mp4"></video> **Lorenz Attractor** |




| | | |
|:---:|:---:|:---:|
| <video src="https://github.com/altunenes/rusty_art/assets/54986652/5a770aae-0a30-4db1-8038-c87c1bc56b9c" type="video/mp4" style="max-width:100px;"></video> **Ulam Sprial** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/71f64338-8a0c-4ccd-9a14-c5c4cceac6b6" type="video/mp4"></video> **Rainbow Gabor Patch via WGPU!** | <video src="https://github.com/altunenes/rusty_art/assets/54986652/248a4a9c-ccae-47cb-97fc-1bb7b25f2be2" type="video/mp4"></video> **Galaxy in WGPU** |
















































