# CYRO

A simple photo editor , but in rust also IT HAS A UPSCALER

## features

- png , jpeg , jpg support 
- crop the images
- backup before editing anything 
- upscale imgs 

## process

it uses eframe and egui for the gui, and the image crate for image loading, processing, and saving. and 
for cropping , it is done by mainly the `crp.rs` in `./src` also abt the upscaler , its a process runs on terminal , 
cyro js handles the command to run it on terminal .

## requirement

- Rust and Cargo
- `realesrgan-ncnn-vulkan` for upscaling (frm aur)
