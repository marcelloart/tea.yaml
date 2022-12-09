# marcel
Dynamic theme library for the iced GUI framework

---

`marcel` introduces a new way to load and manage themes for the [`iced`](https://github.com/iced-rs/iced) GUI framework,
allowing the users to distribute and develop their own themes.

This crate integrates seamlessly with both `iced` in the GUI front and [`serde`](https://github.com/serde-rs/serde) in the Serialization front,
while being agnostic to the serialization format chosen by the user.



## Usage
Add this to you `Cargo.toml`
```
[dependencies]
marcel = "0.1"
```
To get started check out the Github [wiki](https://github.com/micro-rust/marcel/wiki).



## File structure for packaged themes

The file structure of the themes is designed to be easily packaged and highly portable.
The root folder can be stored as a compressed folder (.zip, .rar, etc...) and be used in any
`iced` application through `marcel`.


```
<Theme Name>        # <-- Base folder of the theme
| - theme.xxx       # <-- File named 'theme' with the file format extension (JSON, RON, BON, etc...)
| - theme.meta.xxx  # <-- File named 'theme.meta' with the file format extension (JSON, RON, BON, etc...)
| - img/            # <-- Folder with the images and icons of the theme
| | - img.meta.xxx  # <-- File named 'img.meta' with the file format extension (JSON, RON, BON, etc...)
| | - image.jpg
| | - icon.png
| - font            # <-- Folder with the fonts of the theme
| | - font.ttf
```

```theme.xxx``` contains the full theme contents. This file will be parsed into an `iced` theme.

```theme.meta.xxx``` is an optional file that contains some metadata of the theme such as descriptions and information.
This file is used for theme modification and UI/UX.

```img/``` is the folder which contains all the icons and images referenced in the theme.

```img.meta.xxx``` is an optional file that contains some metadata of the images such as descriptions and information.
This file is used for theme modification and UI/UX.

```font/``` is the folder which contains all the fonts used by the theme.



## License
`marcel` is licensed under the terms of both MIT license and Mozilla Public License (Version 2.0).

See [LICENSE-MIT](https://github.com/micro-rust/marcel/blob/main/LICENSE-MIT) and [LICENSE-MPL2](https://github.com/micro-rust/marcel/blob/main/LICENSE-MPL2) for details.
