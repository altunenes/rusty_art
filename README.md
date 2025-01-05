#  Rusty_Art 🌈 🎨 🦀

[![Rust](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rusty_art/actions/workflows/rust.yml)
[![Build & Release](https://github.com/altunenes/rusty_art/actions/workflows/release.yml/badge.svg)](https://github.com/altunenes/rusty_art/actions/workflows/release.yml)
![banner](https://github.com/altunenes/rusty_art/assets/54986652/c262d693-e7e6-48c1-b4a3-1ef07e4491dc)

I migrated to my own shader engine [cuneus](https://github.com/altunenes/cuneus)

reason: I liked Nannou a lot, I learned Rust thanks to Nannou and awesome devs, but unfortunately it's not being developed anymore. That's why I don't want to stay in very old versions of wgpu so that I can follow recent updates more easily and get more comfortable exports, use atomics, set multi-pass, wasm, hot reload etc... :-) 

**Creative coding with Rust!** 🦀


In this repository, I will create animations and optical illusions that allow for the generation of various patterns in my spare time. So, this repo is consistently updated and current 🙂

Most of my files feature a **graphical user interface (GUI)**, enabling real-time adjustments to the animations and opening up the possibility of creating thousands of unique patterns. With egui, you can also include your own textures or images without needing to hard-code changes.


I hope you enjoy it!


## 🚀 Usage/Installation:


### Easy Setup with Precompiled Binaries

1. Go to the [Releases](https://github.com/altunenes/rusty_art/releases).
2. Download the binary for your operating system (Windows, macOS, or Linux) under the Assets section & Unzip the downloaded file.
3. Run the executable corresponding to the animation you want to enjoy!

This is the simplest way to run animations without any setup or compilation. 🙂


### Compiling from Source

    ```
        git clone https://github.com/altunenes/rusty_art.git
        cd rusty_art
        cargo run --release --bin <filename>
    ```

That's it! If you encounter any issues while opening/running files, feel free to contact me. 🙂


### Shortcuts 🚀

- **<kbd>Spacebar</kbd>**: Hold down to save each frame as a PNG image into the "frames" folder automatically created in your current directory. These can be used later to create high-resolution videos. (for now, only works from source)
- **<kbd>F</kbd>**: Toggle fullscreen mode.
- **<kbd>H</kbd>**: Hide or show the user interface.
- **Mouse Wheel**: Zoom in and out. Note: This feature is not available for all animations.


### Warning ⚠️

I only tested these codes on my notebook with the following specs:

**Windows & Ubuntu:**
- CPU: Ryzen 7 6800h
- Ram: DDR5 16GB
- GPU: Nvidia RTX 3060 mobile

**Apple M3 Air 16GB** 
- (in full screen, some expensive animations could be laggy)

And I must say that some of the files are very computationally intensive. So, if you have a relatively "low-end" computer, you may not be able to run some of the scripts smoothly.
And please don't run the files on your computer if you don't have a good cooling system. I don't want to be responsible for any damage to your computer. 😅

Always open if you have any suggestions or "cheap" tricks (in terms of reducing computational complexity) to improve the performance of the animations. 😊

#### 🖼️ Some Examples:
_Click on the images to open the videos_ 😊

| Attractors | Expensive Mandelbrot | Ernst Chladni's Plate Experiments |
|:---:|:---:|:---:|
| [![Attractors](https://github.com/altunenes/rusty_art/assets/54986652/45c0a523-0785-4d7c-95d9-cdf2e57cd6b9)](https://user-images.githubusercontent.com/54986652/242607093-91bc7605-5223-4eae-a0dc-365e826c0792.mp4) [Code](https://github.com/altunenes/rusty_art/blob/master/src/attractors.rs)|[![mandelbrot](https://github.com/altunenes/rusty_art/assets/54986652/e8f10adc-4d78-48e5-8f43-33f23c7af205)](https://github.com/altunenes/rusty_art/assets/54986652/23711923-3b3f-4818-83c9-64f57d439e24) [Code](https://github.com/altunenes/rusty_art/blob/master/src/expmandelbrotgpu.rs)| [![Erns Chladni's Plate Experiments](https://github.com/altunenes/rusty_art/assets/54986652/6e21c757-ce8c-4205-a542-7d96f37ae73b)](https://github.com/altunenes/rusty_art/assets/54986652/286d5567-d6ac-47ec-8889-d4371173aa7f) [Code](https://github.com/altunenes/rusty_art/blob/master/src/chladniwgpu.rs) |


| Draw something with Fourier Cycloid | Fake Fluid Dynamics | Hilbert Curve Image |
|:---:|:---:|:---:|
| [![Draw something with Fourier Cycloid](https://github.com/altunenes/rusty_art/assets/54986652/0057ff1f-acfc-45c2-9f03-cae7cc6b9a3e)](https://github.com/altunenes/rusty_art/assets/54986652/562d988a-f8f6-438e-9619-42a1794d1534) [Code](https://github.com/altunenes/rusty_art/blob/master/src/dfft.rs) | [![Fake Fluid Dynamics](https://github.com/altunenes/rusty_art/assets/54986652/3e66dc73-2a90-4027-861c-7d9a50b3c6ee)](https://github.com/altunenes/rusty_art/assets/54986652/06298c66-5082-4d21-ba61-3ad1cc300d59) [Code](https://github.com/altunenes/rusty_art/blob/master/src/fluid.rs) | [![Hilbert Curve Image](https://github.com/altunenes/rusty_art/assets/54986652/3a4a4b5a-e5c0-4d9a-aa13-ed3745802bdd)](https://github.com/altunenes/rusty_art/assets/54986652/9cf62ec1-558f-4825-9e4a-e03a67936f18) [Code](https://github.com/altunenes/rusty_art/blob/master/src/hilbertimg.rs) |

| Ray Marching Neuron| Munker Illusion | Neuron Simulation |
|:---:|:---:|:---:|
| [![3D Neuron](https://github.com/altunenes/rusty_art/assets/54986652/f4ceb793-a46b-440d-9bda-5045bebee5d3)](https://github.com/altunenes/rusty_art/assets/54986652/303a4671-4b93-41c4-994f-b8b58edccabf) [Code](https://github.com/altunenes/rusty_art/blob/master/src/3dneuron.rs) | [![Munker Illusion Clock](https://github.com/altunenes/rusty_art/assets/54986652/30bfa484-1ba4-45a3-8978-405ea65a02bb)](https://github.com/altunenes/rusty_art/assets/54986652/13ee00e9-d4f6-4adc-afbc-21bb2085126f) [Code](https://github.com/altunenes/rusty_art/blob/master/src/munkerclock.rs) | [![Neuron Simulation](https://github.com/altunenes/rusty_art/assets/54986652/91c7aea2-182a-4938-9d75-0de6e7178f0e)](https://github.com/altunenes/rusty_art/assets/54986652/e3ad92dd-fead-4778-bf0a-16a882b0f3ff) [Code](https://github.com/altunenes/rusty_art/blob/master/src/neurons.rs) |

| Snowflake | Voronoi Image | Galaxy |
|:---:|:---:|:---:|
| [![Snowflake](https://github.com/altunenes/rusty_art/assets/54986652/67de1335-39d1-4317-9291-3e3a2a3514e6)](https://github.com/altunenes/rusty_art/assets/54986652/ede39f09-ab22-4106-8618-a81d660b4d93) [Code](https://github.com/altunenes/rusty_art/blob/master/src/snowflakewgpu.rs) | [![Voronoi Image](https://github.com/altunenes/rusty_art/assets/54986652/fefda7b1-4da1-4cba-bd55-ce58bad54469)](https://github.com/altunenes/rusty_art/assets/54986652/da40be23-8765-4a04-91a5-63b623332a79) [Code](https://github.com/altunenes/rusty_art/blob/master/src/voronoi.rs) | [![Galaxy](https://github.com/altunenes/rusty_art/assets/54986652/62a4ebee-e9f6-4c47-8e75-1d404f730a39)](https://github.com/altunenes/rusty_art/assets/54986652/248a4a9c-ccae-47cb-97fc-1bb7b25f2be2) [Code](https://github.com/altunenes/rusty_art/blob/master/src/galaxy.rs) |


| Reverse Phi Motion | 🌈 Gabor Patch | Asahi illusion |
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


| Cafewall Illusion | lilac chaser illusion | Peace |
|:---:|:---:|:---:|
| [![Cafewall Illusion](https://github.com/altunenes/rusty_art/assets/54986652/cbd0f3d1-968e-4edc-b1d0-8a9623c58100)](https://user-images.githubusercontent.com/54986652/232924117-17765b32-5da4-4c57-88d5-cdc9eecc7ff4.mp4) [Code](https://github.com/altunenes/rusty_art/blob/master/src/cafe_wall.rs) | [![lilac](https://github.com/altunenes/rusty_art/assets/54986652/4271520d-fb43-4c75-b7ac-405c7ad04d9e)](https://github.com/altunenes/rusty_art/assets/54986652/34c9cb21-5092-462c-b717-65986d710932) [Code](https://github.com/altunenes/rusty_art/blob/master/src/lilac.rs) | [![tree](https://github.com/altunenes/rusty_art/assets/54986652/f576242b-3b95-4674-93a8-acd04dc229de)](https://github.com/altunenes/rusty_art/assets/54986652/ab3e7c33-38f0-4b55-96d2-5b51f8f51c59) [Code](https://github.com/altunenes/rusty_art/blob/master/src/peace.rs) |

| imgblob | Leviant's Optical Illusion | ULAM Spiral |
|:---:|:---:|:---:|
| [![imblob](https://github.com/altunenes/rusty_art/assets/54986652/20a3a0e4-00e0-4696-a18b-87877e4e23f9)](https://github.com/altunenes/rusty_art/assets/54986652/0aeac456-0690-412a-8a18-9a060773f852) [Code](https://github.com/altunenes/rusty_art/blob/master/src/imgblob.rs) | [![Leviant](https://github.com/altunenes/rusty_art/assets/54986652/506ed04b-be7d-4cf1-aabe-c0c9f37aeacf)](https://github.com/altunenes/rusty_art/assets/54986652/55928138-40c6-4306-927a-5a3876d33fce) [Code](https://github.com/altunenes/rusty_art/blob/master/src/leviant.rs) | [![ULAM](https://github.com/altunenes/rusty_art/assets/54986652/9d35dd20-8283-4448-907b-371e36f07b57)](https://github.com/altunenes/rusty_art/assets/54986652/cdaa45ba-74ca-4e06-84c3-e688897985cd) [Code](https://github.com/altunenes/rusty_art/blob/master/src/ulam.rs) |


| alien brain | mandelbulb | Signed Distance Field psychology Ψ |
|:---:|:---:|:---:|
| [![darkclouds](https://github.com/altunenes/rusty_art/assets/54986652/6317e070-7b44-4922-b6f0-f372ddc94fc9)](https://github.com/altunenes/rusty_art/assets/54986652/66504ad9-8bde-4c42-a0fd-793c0ac3ba36) [Code](https://github.com/altunenes/rusty_art/blob/master/src/darkclouds.rs) | [![mandelbulb](https://github.com/user-attachments/assets/1cd7ea57-ba76-4dc0-a05c-d0ba47b06251)](https://github.com/user-attachments/assets/9ea153a4-d76c-45a6-98ca-fde9e5b0ab37) [Code](https://github.com/altunenes/rusty_art/blob/master/src/mandelbulb.rs) | [![blobs](https://github.com/altunenes/rusty_art/assets/54986652/54add6dd-4d67-4290-a45b-56d70d7b63bb)](https://github.com/altunenes/rusty_art/assets/54986652/3d969664-c580-42b9-8999-cd8631177ee5) [Code](https://github.com/altunenes/rusty_art/blob/master/src/psychology.rs) |


| Gaussian Splats: SLIC superpixels [Guide](https://altunenes.github.io/posts/gauss/).| sin julia | psychedelic tunnel |
|:---:|:---:|:---:|
| [![splats](https://github.com/altunenes/rusty_art/assets/54986652/0dfbce4d-984d-4e37-a539-187f0fe0a20e)](https://github.com/altunenes/rusty_art/assets/54986652/bd226177-1c87-43f9-8c27-3a805da10a68) [Code](https://github.com/altunenes/rusty_art/blob/master/src/gaussiansplat.rs) | [![nebula](https://github.com/altunenes/rusty_art/assets/54986652/25515aa2-022d-4b47-9900-5fbddecd82c1)](https://github.com/altunenes/rusty_art/assets/54986652/1a3d041e-d6b2-4780-a591-89924fa5457a) [Code](https://github.com/altunenes/rusty_art/blob/master/src/nebula.rs) | [![tunnel](https://github.com/altunenes/rusty_art/assets/54986652/0c786e0f-d27f-42b8-9ec3-4a5721066e37)](https://github.com/altunenes/rusty_art/assets/54986652/965c4218-932c-4b06-b6e2-a66082944a41) [Code](https://github.com/altunenes/rusty_art/blob/master/src/tunnel.rs) |


| Smooth voronoi | Gabor Lines illusion | Fake tunnel |
|:---:|:---:|:---:|
| [![svoro](https://github.com/altunenes/rusty_art/assets/54986652/a86a4af2-993b-4285-be5a-2275d32b8fb1)](https://github.com/altunenes/rusty_art/assets/54986652/2a95f23b-b9e8-4a2a-a989-48813d4b2cc5) [Code](https://github.com/altunenes/rusty_art/blob/master/src/smoothvoro.rs) | [![nebula](https://github.com/altunenes/rusty_art/assets/54986652/97c275ed-e884-4fea-9574-fab491aa61d8)](https://github.com/altunenes/rusty_art/assets/54986652/87afdc2b-375f-4fe7-97c0-8b23a8cd1f8e) [Code](https://github.com/altunenes/rusty_art/blob/master/src/dottedlines.rs) | [![faketunnel](https://github.com/altunenes/rusty_art/assets/54986652/ee9d6ce4-adb2-4d5e-8553-8c2ecb10eb70)](https://github.com/altunenes/rusty_art/assets/54986652/052d0f26-48e8-49cb-ae9f-71edc6375e08)[Code](https://github.com/altunenes/rusty_art/blob/master/src/faketunnel.rs) |


| Wrapper | Galaxy2  | GPU attractors |
|:---:|:---:|:---:|
| [![wrapper](https://github.com/user-attachments/assets/994dd727-429d-41bb-95f3-5b22cf81149e)](https://github.com/user-attachments/assets/8f385d50-3b5c-4b6d-b571-d5309e9fbfcc) [Code](https://github.com/altunenes/rusty_art/blob/master/src/wrapper.rs) | [![galaxy2](https://github.com/user-attachments/assets/77ff3939-a238-4208-b579-7d82899dda36)](https://github.com/user-attachments/assets/eed83d0a-e130-47f5-96df-7cf172fb93b6) [Code](https://github.com/altunenes/rusty_art/blob/master/src/galaxy2.rs) | [![GPUattractor](https://github.com/user-attachments/assets/0cc2b836-c6de-4dc1-bba5-728f57632ec5)](https://github.com/user-attachments/assets/2b228612-7a85-4f72-bb3f-21fa6ed86476)[Code](https://github.com/altunenes/rusty_art/blob/master/src/GPUattractor.rs) |


| Adelson's 3D Illusion  | Orbit Traps  | Golf |
|:---:|:---:|:---:|
| [![adelson](https://github.com/user-attachments/assets/aa021d43-653c-4246-b3b7-1a1ddd149dc3)](https://github.com/user-attachments/assets/e981e6d6-84fa-4340-9608-3f3b3060f11e) [Code](https://github.com/altunenes/rusty_art/blob/master/src/adelson.rs) | [![orbits](https://github.com/user-attachments/assets/a30ab05b-3015-409d-9aab-d775b5961d83)](https://github.com/user-attachments/assets/2df82bea-0572-4950-843e-0d35cd8b897c) [Code](https://github.com/altunenes/rusty_art/blob/master/src/orbittraps.rs) | [![Golf](https://github.com/user-attachments/assets/ec8e8fc6-40e3-4639-90fb-8e055bf7448b)](https://github.com/user-attachments/assets/fd74fd31-8493-4281-b5ed-35290076d801)[Code](https://github.com/altunenes/rusty_art/blob/master/src/golf.rs) |


| 🎄2025🎄  | ❄️winterflake❄️ | sdrect |
|:---:|:---:|:---:|
| [![2025tree](https://github.com/user-attachments/assets/ed404e21-59dd-467f-a1c6-dddbe6cd8d78)](https://github.com/user-attachments/assets/afcff266-17bf-4036-81fd-c9916ba7f992) [Code](https://github.com/altunenes/rusty_art/blob/master/src/2025tree.rs) | [![winterflake](https://github.com/user-attachments/assets/f2b1804a-865f-4341-81a9-49b0fddd8f26)](https://github.com/user-attachments/assets/26fc5a59-1e8a-4faf-b679-b3c6aafa2ed0) [Code](https://github.com/altunenes/rusty_art/blob/master/src/winterflake.rs) | [![sdrect](https://github.com/user-attachments/assets/36aced15-b948-4da5-bb25-9da4d856139e)](https://github.com/user-attachments/assets/39be6ba4-13d4-4cf3-b4b7-f8e05e50f832)[Code](https://github.com/altunenes/rusty_art/blob/master/src/sdvert.rs) |
