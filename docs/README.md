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



## pkgbuild.yml format:

* `name`: Package name.
* `version`: Package's version.
* `release`: Package's release version, useful when build script need change with same package version.
* `description`: Short description for package.
* `architecture`: Package's architecture. example: x86_64
* `licenses`: Package's licenses.
* `project_url`: Package's url.
* `project_owner`: People that create the project.
* `sources`:  address: package source to download.
* `save_as`: name the package you download.
* `extract`: false or true.
* `extract_to`: leave it empty "" or put the location.
* `security`: todo!
* `dependencies`: run_dependencies, build_dependencies, see examples on how to use this [features](./dependencies.md).
* `prepare`: todo!
* `build`: todo!
* `check`: todo!
* `package`: todo!
* `command`: command for building the source file. Check this to learn about [command](./command.md).
* `deployment`: todo!


## Server:

### Generate server config

    ```bash
    server config
    ```
### Run the server

    ```bash
    server -g
    ```
    note* for the first time, it requires to run -g arg*

## Bin-repo:

- bin-repo use for creating repository to register our package.app after finished build the package.

`To generate a new binary repo.`

    ```bash
    sudo bin-repo create /var/www/repo_name/repo_name.db
    ```
`To add the package.app to the repo`

    ```bash
    sudo bin-repo add /var/www/repo_name/repo_name.db package.app
    ```

`To remove the package.app from the repo`

    ```bash
    sudo bin-repo remove /var/www/repo_name/repo_name.db package_name
    ```

## Pi:

### Generate pkgbuild.yml template.

    ```bash
    pi g
    ```

### Build Port

- After generate the template, you can modify it for the package you want to port.

    ```bash
    pi build
    ```
- After you finish building, there will a new package with a `.app` extenstion. Now it is time to register it to the repo.


    ```bash
    bin-repo add rootfs/var/www/repo_name/repo_name.db package.app
    bin-repo remove rootfs/var/www/repo_name/repo_name.db package_name
    ```

### Install App

    ```bash
    pi install package_name
    ```

### Remove App

    ```bash
    pi remove package_name
    ```


### Update Repo

    ```bash
    pi update
    ```









## pkgbuild.yml variables

- MAKEFLAGS
- PKGNAME
- PKGVER
- PKGREL,
- BASEDIR
- SRCDIR
- PKGDIR
