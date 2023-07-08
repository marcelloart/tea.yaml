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

WARNING: `marcel v0.1.2` is compatible with `iced v0.9.0` and `iced_native v0.10` and with `serde v1`.

See below for a way to load a theme into your application.

```rust
use marcel::{
    theme::{
        Theme,
        serial::Theme as Serial,
    },
};

use std::{
    io::Read,
    fs::File,
    path::PathBuf,
};

fn main() {
    // Buffer to hold the file contents.
    let mut buffer = Vec::new();

    // Open the file and read its contents.
    // Replace '.xxx' with the extension of your file format.
    let mut file = File::open( PathBuf::from("theme.xxx") ).unwrap();
    file.read_to_end(&mut buffer);

    // String to transform to UTF8.
    let string = String::from_utf8(buffer).unwrap();

    // Deserialize.
    // format is the serde compatible format crate (e.g. serde_json)
    let serial: Serial = <format>::from_str(&string).unwrap();

    // Tranform into a theme.
    let mut theme = Theme::new();
    theme.parse(&serial).unwrap();
}
```

To get started check out the Github [wiki](https://github.com/micro-rust/marcel/wiki).


## Theme structure

A `marcel` `Theme` is basically a colelction of `HashMap`s that contain different structures that can
create `iced` compatible `StyleSheet`s. The main structure of the theme can be seen [here](https://github.com/micro-rust/marcel/blob/main/src/theme/mod.rs).

The base structure of a `Theme` is a `Color`. A `Color` is a serializable structure that contains
the basic RGBA values of a color. Building upon that structure is the `Border`, which defines the
border of a widget with a `Color`, a width and a corner radius. The definition of the color is taken as
a string reference to the name of the color wanted. This allows for easy prototyping and theming (e.g.
naming a color as 'base' and another as 'accent' and using them to theme consistently).

```rust
color: {
    "white": Color(255, 255, 255, 1.0),
    "black": Color(  0,   0,   0, 1.0),

    "transparent": Color(  0,   0,   0, 0.0),
},

border: {
    "onepixel-black-square": Border(
        color: "black",
        width: 1.0,
        radius: 0.0,
    ),

    "onepixel-black-round": Border(
        color: "black",
        width: 1.0,
        radius: 2.0,
    ),

    "transparent-round": Border(
        color: "transparent",
        width: 0.0,
        radius: 2.0,
    ),
},
```

All the rest of `StyleSheet`s are based on these two base structures. These `StyleSheets` can be constructed in 3 ways,
which, in `StyleSheet`s with more than one field these methods can be combined.

### 1. Explicit definition
A `StyleSheet` can be constructed as an explicit structure. The structure takes the colors and borders as references.

```rust
"style-1": StyleSheet(
    appearance: Defined(
        background: "red",
        text: "black",
        border: "transparent-round",
    ),
)
```

### 2. Style inheritance
A `StyleSheet`'s fields can be constructed as a copy of another `StyleSheet` field. From all `StyleSheet` of a type,
at least ONE MUST be defined using a combination of methods 1 and 3. When chaining inheritances, the crate currently
accepts only up to a search depth of 10 (from the first `StyleSheet`). Circular dependencies will fail
(e.g. being two styles `x` and `y`, the field `x.a` cannot depend on `y.a` if the field `y.b` depends on `x.b`).

```rust
"style-2": StyleSheet(
    primary: Inherited("style-1"),
    secondary: Inherited("style-3"),
)
```

In this case, `style-2` will copy the `primary` field from `style-1` and the `secondary` field from `style-3`.

### 3. Style fallback
A `StyleSheet` with several `State`s (such as a button that can be active, hovered, pressed and disabled) can have many of
its states be the same. In that case it would not be ergonomic to explicitly write all the fields out, as they would be the same.
For this case, the `None` option allows for default implementations.

```rust
"button": Button(
    active: Defined(State(
        background: Color(0, 0, 0, 1.0),
        text: Color(255, 255, 255, 1.0),
    )),

    hovered: None,

    pressed: Defined(State(
        background: Color(255, 0, 0, 1.0),
        text: Color(0, 0, 255, 1.0),
    )),

    disabled: None,
)
```

In this case the style of the `active` `State` will be copied into the hovered and disabled `State`s.


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


## WIP

See below a list of features that are not implemented yet.

 - Packaged themes
 - Metadata
 - Images
 - Fonts

See below a list of widgets that do not implement a theme yet.

 - `Checkbox`
 - `Radio`
 - `Rule`
 - `Slider`


## Dependencies
See below a list of dependencies of this crate.

 - iced: 0.9
 - iced_native: 0.10
 - serde: 1 (feaures = [derive])


## Contributing

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under either the MIT license or the Mozilla Public License Version 2.0, without any additional terms and conditions.


## License
`marcel` is licensed under the terms of both MIT license and Mozilla Public License (Version 2.0).

See [LICENSE-MIT](https://github.com/micro-rust/marcel/blob/main/LICENSE-MIT) and [LICENSE-MPL2](https://github.com/micro-rust/marcel/blob/main/LICENSE-MPL2) for details.
