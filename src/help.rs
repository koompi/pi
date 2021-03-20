use colored::Colorize;

pub fn help(option: &str) {
    match option {
        "build" => help_build(),
        "install" => help_install(),
        "remove" => help_remove(),
        "update" => help_update(),
        _ => {
            println!("\n{}:", "USAGE".green().bold());
            help_build();
            help_install();
            help_remove();
            help_update();
        }
    }
}

fn help_build() {
    println!("\n{}", "BUILD:".magenta());
    println!(
        "- Build application from source presuming there is pkgbuild.yml in current directory."
    );
    println!("=> {}", "store build".blue().bold(),);
    println!("- Build application from specified package file.");
    println!("=> {}", "store build path_to_file.yml".blue().bold(),);
}

fn help_install() {
    println!("\n{}", "INSTALL:".magenta());
    println!("- Installation from store");
    println!("=> {}", "store install app_1 app_n".blue().bold(),);
    println!("- Installation from file");
    println!(
        "=> {}",
        "store install --file app_1.app app_n.app".blue().bold(),
    );
}

fn help_remove() {
    println!("\n{}", "REMOVE:".magenta());
    println!("- Remove give applications");
    println!("=> {}", "store remove app_1 app_n".blue().bold(),);
}

fn help_update() {
    println!("\n{}", "UPDATE:".magenta());
    println!("- Update all install applications");
    println!("=> {}", "store update".blue().bold(),);
}
