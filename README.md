<a id="readme-top"></a>

<div align="center">

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![Crates.io][crates-shield]][crates-url]
[![Downloads][crates-dl-shield]][crates-dl-url]

<br />

<h3 align="center">projectM-rs</h3>

  <p align="center">
    Safe-wrapper bindings for <a href="https://github.com/projectM-visualizer/projectm" target="_blank">ProjectM</a>
    <br />
    <br />
    <a href="https://docs.rs/projectm-rs/latest" target="_blank">Docs</a>
    ·
    <a href="https://github.com/projectM-visualizer/frontend-sdl2-rust" target="_blank">Example</a>
    ·
    <a href="https://github.com/projectM-visualizer/projectm-rs/issues" target="_blank">Report Bug</a>
    ·
    <a href="https://github.com/projectM-visualizer/projectm-rs/issues" target="_blank">Request Feature</a>
  </p>
</div>

<br />

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#support">Support</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<br />

<!-- GETTING STARTED -->
## Getting Started

To get this crate up and running properly, you'll need to install some prerequisites.

### Prerequisites

Depending on the OS/distribution and packaging system, libraries might be split into separate packages with binaries and
development files. To build projectM, both binaries and development files need to be installed.

#### General build dependencies for all platforms:

* [**Rust**](https://www.rust-lang.org/tools/install)
* A working build toolchain.
* [**CMake**](https://cmake.org/): Used to generate platform-specific build files.
* **OpenGL**: 3D graphics library. Used to render the visualizations.
* or **GLES3**: OpenGL libraries for embedded systems, version 3. Required to build projectM on Android devices,
  Raspberry Pi, Emscripten and the Universal Windows Platform.

#### Only relevant for Windows:

* [**vcpkg**](https://github.com/microsoft/vcpkg): C++ Library Manager for Windows. _Optional_, but recommended to
  install the aforementioned library dependencies.
* [**GLEW**](http://glew.sourceforge.net/): The OpenGL Extension Wrangler Library. Only required if using CMake to
  configure the build, the pre-created solutions use a bundled copy of GLEW.
<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

```
# Cargo.toml

[dependencies]
projectm-rs = { version = "1.0", features = [] }   # Available features: playlist
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- EXAMPLES -->
## Example
You can easily try it out yourself, by running the command below.

```
cargo run --example sdl
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the LGPL-2.1 license. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- SUPPORT -->
## Support

[![Discord][discord-shield]][discord-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Blaquewithaq (Discord: SoFloppy#1289) - [@anomievision](https://twitter.com/anomievision) - anomievision@gmail.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/projectM-visualizer/projectm-rs.svg?style=for-the-badge
[contributors-url]: https://github.com/projectM-visualizer/projectm-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/projectM-visualizer/projectm-rs.svg?style=for-the-badge
[forks-url]: https://github.com/projectM-visualizer/projectm-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/projectM-visualizer/projectm-rs.svg?style=for-the-badge
[stars-url]: https://github.com/projectM-visualizer/projectm-rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/projectM-visualizer/projectm-rs.svg?style=for-the-badge
[issues-url]: https://github.com/projectM-visualizer/projectm-rs/issues
[license-shield]: https://img.shields.io/github/license/projectM-visualizer/projectm-rs.svg?style=for-the-badge
[license-url]: https://github.com/projectM-visualizer/projectm-rs/blob/master/LICENSE
[crates-shield]: https://img.shields.io/crates/v/projectm-rs?style=for-the-badge
[crates-url]: https://crates.io/crates/projectm-rs
[crates-dl-shield]: https://img.shields.io/crates/d/projectm-rs?style=for-the-badge
[crates-dl-url]: https://crates.io/crates/projectm-rs
[discord-shield]: https://img.shields.io/discord/737206408482914387?style=for-the-badge
[discord-url]: https://discord.gg/7fQXN43n9W
