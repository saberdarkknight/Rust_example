# Tauri



## Content

The example of building PC application with ![Tauri framework](https://v2.tauri.app/).






## Step

### Setup Framework

1.  Install development dependencies with `cargo install create-tauri-app --locked`

2. Create a project with `cargo create-tauri-app`

2.1 You will need to give a name to the project.
Then, you will also need to give the identifier and select the programming language as shown in the image below.

<div align="center">
<div>
<img src="https://github.com/saberdarkknight/Rust_example/blob/main/Tauri/fig/tauri_1.png" width="70%" />
</div>
</div>

2.2 The next step is to select the preferred UI template.

<div align="center">
<div>
<img src="https://github.com/saberdarkknight/Rust_example/blob/main/Tauri/fig/tauri_2.png" width="70%" />
</div>
</div>

2.3 With the previous selection, you can start coding and build the application in different OS.


<div align="center">
<div>
<img src="https://github.com/saberdarkknight/Rust_example/blob/main/Tauri/fig/tauri_2.png" width="70%" />
</div>
</div>



3. Debug the project with `cargo tauri dev`


4. Build the project with `cargo tauri build`.
In `./target/release`, you will find the executable file (e.g., `.exe` file for Windows)



## Warning

###  Attempt to access communication port denied

The port may be occupied by another process.
You need to modify the port definition defined in `./src-tauri/tauri.conf.json`


### The bundle identifier string must contain only alphanumeric characters (A-Z, a-z, and 0-9), hyphens (-), and periods (.).

This means the name of the application should only contain characters (A-Z, a-z, and 0-9), hyphens (-), and periods (.)
You should check the `identifier` setting in `./src-tauri/tauri.conf.json`


