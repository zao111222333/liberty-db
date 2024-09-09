use clap::App;
use liberty2json::convert_lib;
use log::info;
use pretty_env_logger::init;
use std::path::Path;
fn main() {
    init();
    // from clap examples
    let matches = App::new("lib2json")
        .version("0.1.0")
        .author("Eric Hsu <zhenyuxu163@163.com>")
        .about("Convert liberty file to json")
        .args_from_usage(
            "
			<INPUT> 	'Sets the input liberty file'
			-o --output=[FOLDER] 'Sets a output folder'

			",
        )
        .get_matches();

    if let Some(source) = matches.value_of("INPUT") {
        let input = Path::new(source);
        let output_path = if let Some(desti) = matches.value_of("FOLDER") {
            Path::new(desti)
        } else {
            info!("Not Set output path,will use the path of liberty file as default path");
            input
                .parent()
                .expect("Cannot get parent path of liberty file")
        };
        convert_lib(input, output_path).expect("Failed to generate json");
    }
}
