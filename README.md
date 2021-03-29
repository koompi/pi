# PI Package Manager

## Developmet

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
