<h1 align="Center"> Documentation

<h3 align="center">PI Package Manager </h1>




## Description

Pi is a package manager built in order to manage packages for KOOMPI Linux. This package manager
is fully written in Rust. The package building script is difference from the others. That why we believe at some point this package manager will stand out. Packages are built and installed into a temporary location using DESTDIR method and are afterwards generate into an *.app.

Installing the *.app means it is extracted into real system. After that all files extracted is recorded into an database. So Pi will track all installed files. Pi can automatically resolve dependencies order. Pi
reads the build script (pkgbuild.yml) in order to get all necessary variables and functions before building
them.

## pkgbuild.yml

`pkgbuild.yml` is build script sourced by `Pi Package Manager` to build the package.
The example of `pkgbuild.yml` as follows:

	---
	metadata:
	    name: neofetch
	    version: 7.1.0
	    release: 1
	    description: A CLI system information tool written in BASH that supports displaying images.
	    architecture:
	        - x86_64
	    licenses:
	        - MIT
	    project_url:
	        - https://github.com/dylanaraps/neofetch
	    project_ownder: []
	sources:
	    - address: "git://github.com/dylanaraps/neofetch"
	      save_as: neofetch
	      extract: false
	      extract_to: ""
	security: ~
	dependencies: ~
	prepare: ~
	build: ~
	check: ~
	package:
	    commands:
	        - cd "${SRCDIR}/${PKGNAME}"
	        - make DESTDIR=${PKGDIR} install
	deployment: ~



#### pkgbuild.yml format:

* `name`: Package name.
* `version`: Package's version.
* `release`: Package's release version, useful when build script need change with same package version.
* `description`: Short description for package.
* `architecture`: Package's architecture. example: x86_64
* `licenses`: Package's licenses.
* `project_url`: Package's url.
* `project_owner`: People that create the project
* `noextract`: Specify file no need to extract, separate with space.
* `nostrip`: list file to avoid strip, can use regex
* `source`: Package's source urls, separate with space, can use as `<new-source-name>::<source-url>` to save source file with different name (see `spkgbuild` example).















1. Building services assuming you are already familiar with Rust.

    ```bash
    cargo build --bin pi
    cargo build --bin server
    cargo build --bin bin-repo
    ```

2. Generate server config

    ```bash
    cargo run --bin server -- config
    ```

3. Run the server

    ```bash
    cargo run --bin server
    ```

4. Generate pkgbuild.yml template.

    ```bash
    cargo run --bin pi -- g
    ```

5. Fill in you pkgbuild file and copy it to `rootfs/tmp` and start building.

    ```bash
    cargo run --bin pi -- b
    ```

6. After you finish building, there will a new package with a `.app` extenstion. Now it is time to register it to the repo.

    1. First you need to generate a new binary repo.

        ```bash
        cargo run --bin bin-repo -- create rootfs/var/www/repo_name/repo_name.db        # generate a new repo
        ```

    2. Add the `package.app` to the repo

        ```bash
        cargo run --bin bin-repo -- add rootfs/var/www/repo_name/repo_name.db package.app
        cargo run --bin bin-repo -- remove rootfs/var/www/repo_name/repo_name.db package_name
        ```

7. Now it is time to run the package manager

    ```bash
    cargo run --bin pi -- update
    cargo run --bin pi -- install the_app_name
    cargo run --bin pi -- remove the app_name
    ```
