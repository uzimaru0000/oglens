<h1 align="center">:search: OGLens</h1>

<h2 align="center"><em>
Tools for viewing OGP
</em></h2>

[![](https://img.shields.io/github/license/uzimaru0000/oglens?style=for-the-badge)](https://github.com/uzimaru0000/oglens/blob/master/LICENSE)
[![](https://img.shields.io/github/v/release/uzimaru0000/oglens?style=for-the-badge)](https://github.com/uzimaru0000/oglens/releases/latest)
![](https://img.shields.io/github/downloads/uzimaru0000/oglens/total?style=for-the-badge)

```bash
$ curl https://ogp.me | og
+-----------------+-----------------------------------------------------------------------------------------+
| property        | content                                                                                 |
+-----------------+-----------------------------------------------------------------------------------------+
| og:title        | Open Graph protocol                                                                     |
+-----------------+-----------------------------------------------------------------------------------------+
| og:type         | website                                                                                 |
+-----------------+-----------------------------------------------------------------------------------------+
| og:url          | https://ogp.me/                                                                         |
+-----------------+-----------------------------------------------------------------------------------------+
| og:image        | https://ogp.me/logo.png                                                                 |
+-----------------+-----------------------------------------------------------------------------------------+
| og:image:type   | image/png                                                                               |
+-----------------+-----------------------------------------------------------------------------------------+
| og:image:width  | 300                                                                                     |
+-----------------+-----------------------------------------------------------------------------------------+
| og:image:height | 300                                                                                     |
+-----------------+-----------------------------------------------------------------------------------------+
| og:image:alt    | The Open Graph logo                                                                     |
+-----------------+-----------------------------------------------------------------------------------------+
| og:description  | The Open Graph protocol enables any web page to become a rich object in a social graph. |
+-----------------+-----------------------------------------------------------------------------------------+
```

## How to install

### For MacOS

```bash
$ brew install uzimaru0000/tap/oglens
```

### Use cargo

```bash
$ cargo install oglens
```

## How to use

### USAGE

```
og [OPTIONS] [PATH]
```

### OPTIONS

```
-h, --help
        Print help information

-j, --json
        Displayed in JSON format

-p, --prefix <PREFIX>
        Prefix settings other than OG. Specify key separated by colons. ex:
        --prefix=twitter:name

-V, --version
        Print version information
```

### ARGS

```
<PATH>  HTML file path
```
