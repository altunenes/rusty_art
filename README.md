#  Rusty_Art 🌈 🎨

[![Rust](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml)

![banner](https://github.com/altunenes/rusty_art/assets/54986652/c262d693-e7e6-48c1-b4a3-1ef07e4491dc)


**Creative coding with Rust!** 🦀


In this repository, I will create animations and optical illusions that allow for the generation of various patterns in my spare time. So, this repo is consistently updated and current 😄

Most of my files feature a **graphical user interface (GUI)**, enabling real-time adjustments to the animations and opening up the possibility to create thousands of unique patterns. 


I hope you enjoy it!


## 🚀 Usage/Installation:

*This section is intended for those who are new to GitHub or Rust and may not be familiar with these tools.*

1- Install [Rust Programming Language](https://www.rust-lang.org/tools/install)

2- Click on the green "Code" button at the top right of this page. Then, select "Download ZIP" from the dropdown menu. After the ZIP file finishes downloading, extract it to a folder on your computer.


3- Open the terminal (also known as command prompt or cmd) on your computer. You can simply open it in the folder "rusty_art" by right-clicking on the folder and selecting "Open in Terminal".


To run the files, in the root directory of the project, type the commands in the following style (Shader names are the same as rust backend; no need to use extensions)

    ```bash
        cargo run --release --bin <filename>
    ```

That's it! If you encounter any issues while opening/running files, feel free to contact me. 😊


### Saving Frames 📸

To create high-resolution videos, you can save each frame as a PNG image by holding down the <kbd>spacebar</kbd> while the animations are running. This will save each frame in a folder called "frames" which will be created automatically in your current directory (Yes for all files, some of them also work with zoom with mouse wheels, but not all).
Once you've saved all the frames you want, you can create a video file by copying the images to a folder and running the following command in that folder:

    
    ```bash
        ffmpeg -r 60 -f image2 -start_number 10000 -i %d.png -vf "scale=-2:990,format=yuv420p" -vcodec libx264 -crf 10 output.mp4
    ```

This command will use the images in the "frames" folder to create a video file named "output.mp4" in the same folder. The video will be encoded with the libx264 codec at a constant rate factor of 10 and with a pixel format of yuv420p. The frame rate will be set to 60 frames per second (-r 60).

Note: You need to install [ffmpeg](https://ffmpeg.org/) to create videos.


### Warning ⚠️

I only tested these codes on my notebook with the following specs:

CPU: Ryzen 7 6800h
Ram: DDR5 16GB
GPU: Nvidia RTX 3060 mobile

And I must say that some of the files are very computationally intensive. So, if you have a relatively "low-end" computer, you may not be able to run some of the scripts smoothly. I'm sorry about that. 😔 
And please don't run the files on your computer if you don't have a good cooling system. I don't want to be responsible for any damage to your computer. 😅

Always open if you have any suggestions or "cheap" tricks (in terms of reducing computational complexity) to improve the performance of the scripts. 😊

Animations that are computationally intensive (in my opinion):
- peace2 and peace3
- mandelbrot (both CPU and GPU versions)
- pixelrain
- attractors
- gabor (CPU version)

#### 🖼️ Some Examples:
_Click on the images to open the videos_ 😊

| Attractors | Expensive Mandelbrot | Ernst Chladni's Plate Experiments |
|:---:|:---:|:---:|
| [![Attractors](https://github.com/altunenes/rusty_art/assets/54986652/45c0a523-0785-4d7c-95d9-cdf2e57cd6b9)](https://user-images.githubusercontent.com/54986652/242607093-91bc7605-5223-4eae-a0dc-365e826c0792.mp4) [Code](https://github.com/altunenes/rusty_art/blob/master/src/attractors.rs)|[![mandelbrot](https://github.com/altunenes/rusty_art/assets/54986652/e8f10adc-4d78-48e5-8f43-33f23c7af205)](https://github.com/altunenes/rusty_art/assets/54986652/23711923-3b3f-4818-83c9-64f57d439e24) [Code](https://github.com/altunenes/rusty_art/blob/master/src/expmandelbrotgpu.rs)| [![Erns Chladni's Plate Experiments](https://github.com/altunenes/rusty_art/assets/54986652/6e21c757-ce8c-4205-a542-7d96f37ae73b)](https://github.com/altunenes/rusty_art/assets/54986652/286d5567-d6ac-47ec-8889-d4371173aa7f) [Code](https://github.com/altunenes/rusty_art/blob/master/src/chladniwgpu.rs) |


| Draw something with Fourier Cycloid | Fake Fluid Dynamics | Hilbert Curve Image |
|:---:|:---:|:---:|
| [![Draw something with Fourier Cycloid](https://github.com/altunenes/rusty_art/assets/54986652/0057ff1f-acfc-45c2-9f03-cae7cc6b9a3e)](https://github.com/altunenes/rusty_art/assets/54986652/562d988a-f8f6-438e-9619-42a1794d1534) [Code](https://github.com/altunenes/rusty_art/blob/master/src/dfft.rs) | [![Fake Fluid Dynamics](https://github.com/altunenes/rusty_art/assets/54986652/3e66dc73-2a90-4027-861c-7d9a50b3c6ee)](https://github.com/altunenes/rusty_art/assets/54986652/06298c66-5082-4d21-ba61-3ad1cc300d59) [Code](https://github.com/altunenes/rusty_art/blob/master/src/fluid.rs) | [![Hilbert Curve Image](https://github.com/altunenes/rusty_art/assets/54986652/3a4a4b5a-e5c0-4d9a-aa13-ed3745802bdd)](https://github.com/altunenes/rusty_art/assets/54986652/9cf62ec1-558f-4825-9e4a-e03a67936f18) [Code](https://github.com/altunenes/rusty_art/blob/master/src/hilbertimg.rs) |


| Mandelbrot Set | Munker Illusion | Neuron Simulation |
|:---:|:---:|:---:|
| [![Mandelbrot Set](https://github.com/altunenes/rusty_art/assets/54986652/cca2b542-d246-477b-9092-a70886048ea2)](https://github.com/altunenes/rusty_art/assets/54986652/1476cb32-2726-4706-a02d-ce4bbd4e7f6e) [Code](https://github.com/altunenes/rusty_art/blob/master/src/mandelbrotgpu.rs) | [![Munker Illusion Clock](https://github.com/altunenes/rusty_art/assets/54986652/30bfa484-1ba4-45a3-8978-405ea65a02bb)](https://github.com/altunenes/rusty_art/assets/54986652/13ee00e9-d4f6-4adc-afbc-21bb2085126f) [Code](https://github.com/altunenes/rusty_art/blob/master/src/munkerclock.rs) | [![Neuron Simulation](https://github.com/altunenes/rusty_art/assets/54986652/91c7aea2-182a-4938-9d75-0de6e7178f0e)](https://github.com/altunenes/rusty_art/assets/54986652/e3ad92dd-fead-4778-bf0a-16a882b0f3ff) [Code](https://github.com/altunenes/rusty_art/blob/master/src/neurons.rs) |


| Snowflake | Voronoi Image | Galaxy |
|:---:|:---:|:---:|
| [![Snowflake](https://github.com/altunenes/rusty_art/assets/54986652/67de1335-39d1-4317-9291-3e3a2a3514e6)](https://github.com/altunenes/rusty_art/assets/54986652/ede39f09-ab22-4106-8618-a81d660b4d93) [Code](https://github.com/altunenes/rusty_art/blob/master/src/snowflakewgpu.rs) | [![Voronoi Image](https://github.com/altunenes/rusty_art/assets/54986652/fefda7b1-4da1-4cba-bd55-ce58bad54469)](https://github.com/altunenes/rusty_art/assets/54986652/da40be23-8765-4a04-91a5-63b623332a79) [Code](https://github.com/altunenes/rusty_art/blob/master/src/voronoi.rs) | [![Galaxy](https://github.com/altunenes/rusty_art/assets/54986652/62a4ebee-e9f6-4c47-8e75-1d404f730a39)](https://github.com/altunenes/rusty_art/assets/54986652/248a4a9c-ccae-47cb-97fc-1bb7b25f2be2) [Code](https://github.com/altunenes/rusty_art/blob/master/src/galaxy.rs) |


| Reverse Phi Motion | Gabor Patch | Asahi illusion |
|:---:|:---:|:---:|
| [![Reverse Phi](https://github.com/altunenes/rusty_art/assets/54986652/1dd779e1-075a-400a-8dcc-f7b031d8b912)](https://github.com/altunenes/rusty_art/assets/54986652/b5ddb9e6-2504-4f26-9ca3-ad5227ea4bca) [Code](https://github.com/altunenes/rusty_art/blob/master/src/pdiamond.rs) | [![Gabor Patch](https://github.com/altunenes/rusty_art/assets/54986652/702b7a06-5a11-4728-8657-ec7d384302c6)](https://github.com/altunenes/rusty_art/assets/54986652/23b642e1-0321-43c5-bcb5-01b9ee6051c8) [Code](https://github.com/altunenes/rusty_art/blob/master/src/gaborwgpu.rs) | [![Asahi illusion](https://github.com/altunenes/rusty_art/assets/54986652/77ffb57f-d9ba-4ba0-b567-8bcb9cbd4dfa)](https://github.com/altunenes/rusty_art/assets/54986652/04b54dbf-f656-420b-8234-026589a82be3) [Code](https://github.com/altunenes/rusty_art/blob/master/src/asahi2.rs) |


| Love WGPU | Lorenz System | Neural Net |
|:---:|:---:|:---:|
| [![Love WGPU](https://github.com/altunenes/rusty_art/assets/54986652/777106a7-a621-433b-8f4b-641ad771fe0d)](https://github.com/altunenes/rusty_art/assets/54986652/3f357272-6c5e-4733-b047-2ec27ce12630) [Code](https://github.com/altunenes/rusty_art/blob/master/shaders/lovewgpu.wgsl) | [![Lorenz](https://github.com/altunenes/rusty_art/assets/54986652/beaf5d59-1847-4ae7-bc6b-0449d34bd20c)](https://github.com/altunenes/rusty_art/assets/54986652/37a44664-69c9-464f-86b0-a35cd9efeeca) [Code](https://github.com/altunenes/rusty_art/blob/master/src/lorenz.rs) | [![Neural Net](https://github.com/altunenes/rusty_art/assets/54986652/75743a83-8262-4fea-8b21-a11dd280b123)](https://github.com/altunenes/rusty_art/assets/54986652/9727b58f-9fca-416b-a61a-5fc4f3a82e27) [Code](https://github.com/altunenes/rusty_art/blob/master/src/neuralnet.rs) |


| Pinna illusion| oscillation | lensing |
|:---:|:---:|:---:|
| [![Pinna](https://github.com/altunenes/rusty_art/assets/54986652/e4413a32-b28f-47e2-9974-f21d12ab8340)](https://github.com/altunenes/rusty_art/assets/54986652/6d39f14e-9529-4cbe-9d82-5fb675437a43) [Code](https://github.com/altunenes/rusty_art/blob/master/src/pina.rs) | [![oscillation](https://github.com/altunenes/rusty_art/assets/54986652/e55c3013-30ed-4f02-bbbb-3d9a4a7b39d2)](https://github.com/altunenes/rusty_art/assets/54986652/47777e12-0c7d-4553-b3e4-fe2f54a38b8b) [Code](https://github.com/altunenes/rusty_art/blob/master/src/oscillation.rs) | [![imlens](https://github.com/altunenes/rusty_art/assets/54986652/ba40f8e3-78d2-410a-b569-6c231405a9c7)](https://github.com/altunenes/rusty_art/assets/54986652/03e7ef39-1b6f-4ed1-9c48-71b312db98a1) [Code](https://github.com/altunenes/rusty_art/blob/master/src/imlenswgpu.rs) |


| fourier | sinh | tree |
|:---:|:---:|:---:|
| [![fourier](https://github.com/altunenes/rusty_art/assets/54986652/c36d78a1-0e6d-4a20-b360-179011e2d5d2)](https://github.com/altunenes/rusty_art/assets/54986652/178d3512-3f04-4ec2-9b24-71a2fde02fbf) [Code](https://github.com/altunenes/rusty_art/blob/master/src/fourier.rs) | [![sinh](https://github.com/altunenes/rusty_art/assets/54986652/b29e196b-91bb-4211-8544-1a18655d8951)](https://github.com/altunenes/rusty_art/assets/54986652/7a3e27cc-34a1-41a3-865c-acc3167a5140) [Code](https://github.com/altunenes/rusty_art/blob/master/src/sinh.rs) | [![tree](https://github.com/altunenes/rusty_art/assets/54986652/2a201ed5-3047-4497-b3ec-2153e5ee23e1)](https://github.com/altunenes/rusty_art/assets/54986652/dabc1608-236d-4134-affe-136690952420) [Code](https://github.com/altunenes/rusty_art/blob/master/src/tree.rs) |


| Cafewall Illusion | lilac chaser illusion | Peace2 |
|:---:|:---:|:---:|
| [![Cafewall Illusion](https://github.com/altunenes/rusty_art/assets/54986652/cbd0f3d1-968e-4edc-b1d0-8a9623c58100)](https://user-images.githubusercontent.com/54986652/232924117-17765b32-5da4-4c57-88d5-cdc9eecc7ff4.mp4) [Code](https://github.com/altunenes/rusty_art/blob/master/src/cafe_wall.rs) | [![lilac](https://github.com/altunenes/rusty_art/assets/54986652/4271520d-fb43-4c75-b7ac-405c7ad04d9e)](https://github.com/altunenes/rusty_art/assets/54986652/34c9cb21-5092-462c-b717-65986d710932) [Code](https://github.com/altunenes/rusty_art/blob/master/src/lilac.rs) | [![tree](https://github.com/altunenes/rusty_art/assets/54986652/f576242b-3b95-4674-93a8-acd04dc229de)](https://github.com/altunenes/rusty_art/assets/54986652/ab3e7c33-38f0-4b55-96d2-5b51f8f51c59) [Code](https://github.com/altunenes/rusty_art/blob/master/src/peace2.rs) |

| imgblob | Leviant's Optical Illusion | ULAM Spiral |
|:---:|:---:|:---:|
| [![imblob](https://github.com/altunenes/rusty_art/assets/54986652/20a3a0e4-00e0-4696-a18b-87877e4e23f9)](https://github.com/altunenes/rusty_art/assets/54986652/0aeac456-0690-412a-8a18-9a060773f852) [Code](https://github.com/altunenes/rusty_art/blob/master/src/imgblob.rs) | [![Leviant](https://github.com/altunenes/rusty_art/assets/54986652/506ed04b-be7d-4cf1-aabe-c0c9f37aeacf)](https://github.com/altunenes/rusty_art/assets/54986652/55928138-40c6-4306-927a-5a3876d33fce) [Code](https://github.com/altunenes/rusty_art/blob/master/src/leviant.rs) | [![ULAM](https://github.com/altunenes/rusty_art/assets/54986652/9d35dd20-8283-4448-907b-371e36f07b57)](https://github.com/altunenes/rusty_art/assets/54986652/cdaa45ba-74ca-4e06-84c3-e688897985cd) [Code](https://github.com/altunenes/rusty_art/blob/master/src/ulam.rs) |
