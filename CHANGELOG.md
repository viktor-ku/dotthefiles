# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## 0.1.0 (2020-07-27)


### Features

* spawn sudo proc when additional perms required ([#15](https://github.com/dotthefiles/dotthefiles/issues/15)) ([0a12303](https://github.com/dotthefiles/dotthefiles/commit/0a123035ea188101a5fadd87aef795ea3a5d2f64))
* **config:** do not parse the file-level `from` field ([147406e](https://github.com/dotthefiles/dotthefiles/commit/147406eed64cc3cd3e3c4d7ceeb3b6828ead5ba4))
* **main:** actually link files and print result of what happened ([8dd914d](https://github.com/dotthefiles/dotthefiles/commit/8dd914d524e49df0dc72e11a49192a4d4a5f87f6))
* **main:** add --list option to print out dotfiles it would link (like a dry-run) ([c94a0e5](https://github.com/dotthefiles/dotthefiles/commit/c94a0e540ec517d6948527dcf6abf5e5d0cd37a0))
* **main:** ask the user if he wants to proceed with the given map ([de3ac31](https://github.com/dotthefiles/dotthefiles/commit/de3ac318f77a322c112c82a99e266799e049e510))
* **main:** print list of the files the program would like to link ([cfecb73](https://github.com/dotthefiles/dotthefiles/commit/cfecb734225ec06760a7ec1421d38e15b5ac5cd4))
* **mapping:** accurately parse `from` block-level field ([44cb695](https://github.com/dotthefiles/dotthefiles/commit/44cb695e0e055636e01d13032f5117e5edc5837f))
* **mapping:** look for dotfiles under the current OS directory name ([fb4412d](https://github.com/dotthefiles/dotthefiles/commit/fb4412dc4aadcdc59426e240f6facacce8f8cc73))
* **mapping:** look for files in `./` instead of the `any/` for `any` target ([08acd24](https://github.com/dotthefiles/dotthefiles/commit/08acd246131b9739d0db95f9859bc35e3e4a732f))
* **mapping:** make `any` target a higher priority over others ([bb638b3](https://github.com/dotthefiles/dotthefiles/commit/bb638b3ff593c79d865b3602a1a19eef65892742))
* **mapping:** resolve `~` to home directory as well as append the rest of the path ([8cd5a46](https://github.com/dotthefiles/dotthefiles/commit/8cd5a46d28808eebfb9940f0bebd9067f69fb23a))
* **mapping:** treat it like `any` target when `target` property is omitted ([e7b1239](https://github.com/dotthefiles/dotthefiles/commit/e7b1239dd7097ae879abd1a16b4a3c161a6314ae))
* assume linux distros to be a linux ([1b6bd2b](https://github.com/dotthefiles/dotthefiles/commit/1b6bd2bd0f3d52d2a5ef8b494182a936e2ded8bf))
* **mapping:** search `any` target under the `any` directory ([aada31c](https://github.com/dotthefiles/dotthefiles/commit/aada31c4245395306279a834009d12459e247ba7))
