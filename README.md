# delete_face

![INPUT](example/input/scientist/scientists.jpg)

![OUTPUT](example/output/scientist/DF_scientists.jpg)

> Delete face from entered image

- **[SEETAFACE C++](https://github.com/seetaface/SeetaFaceEngine/tree/master/FaceDetection)**
- **[RustFace](https://github.com/atomashpolskiy/rustface)**

## use

Make 'input' directory and put the picture you want in the 'input' folder.

if you enter the "test2", set the image path to "input/test2/" and create the "output/test2/" folder to save edited photos.

```bash
$ cargo build --release && ./target/release/delete_face



Input image path: test2

------------------------------------------
image path is: input/test2
------------------------------------------
image count: 12

1/12
input/test2/IMG_1351.jpg
Found 394 faces in 19012 ms
Saved result to ./output/test2/DF_IMG_1351.jpg

2/12
input/test2/IMG_1355.JPG
Found 33 faces in 1196 ms
Saved result to ./output/test2/DF_IMG_1355.JPG

3/12
input/test2/IMG_0894.JPG
Found 7 faces in 7805 ms
Saved result to ./output/test2/DF_IMG_0894.JPG

4/12
input/test2/IMG_1181.JPG
Found 11 faces in 469 ms
Saved result to ./output/test2/DF_IMG_1181.JPG

5/12
input/test2/IMG_0841.JPG
Found 6 faces in 5827 ms
Saved result to ./output/test2/DF_IMG_0841.JPG

6/12
input/test2/IMG_3513.jpg
Found 67 faces in 5680 ms
Saved result to ./output/test2/DF_IMG_3513.jpg

7/12
input/test2/IMG_0233.jpg
Found 24 faces in 634 ms
Saved result to ./output/test2/DF_IMG_0233.jpg

8/12
input/test2/IMG_0270.jpeg
Found 16 faces in 2078 ms
Saved result to ./output/test2/DF_IMG_0270.jpeg

9/12
input/test2/IMG_0365.jpeg
Found 15 faces in 589 ms
Saved result to ./output/test2/DF_IMG_0365.jpeg

10/12
input/test2/IMG_0332.jpeg
Found 16 faces in 2576 ms
Saved result to ./output/test2/DF_IMG_0332.jpeg

11/12
input/test2/IMG_2142.png
Found 50 faces in 1688 ms
Saved result to ./output/test2/DF_IMG_2142.png

12/12
input/test2/IMG_2709.jpeg
Found 11 faces in 4308 ms
Saved result to ./output/test2/DF_IMG_2709.jpeg

------------------------------------------
Done!
```

## example file tree

```text
delete_face/
├── src/
│   └── main.rs
├── input/
│   └── test2/
│       ├── test1.jpg
│       ├── test2.png
│       └── test3.jpeg
├── output/
│   └── test2/
│       ├── DF_test1.jpg
│       ├── DF_test2.png
│       └── DF_test3.jpeg
├── model/
│   └── seeta_fd_frontal_v1.0.bin
├── Cargo.toml
├── Cargo.lock
└── target/
    ├── debug
    └── release/
        └── delete_face
```
