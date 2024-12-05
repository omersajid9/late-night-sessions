use tauri::api::cli;

#[tauri::command]
fn my_command(args: &str) {
    println!("Running command with args: {}", args);
    // Your command logic here
}

fn main() {
    cli::init()
        .setup(|app| {
            app.add_command(
                cli::CommandExt::new("my-command")
                    .alias("mc")
                    .action(my_command),
            );
            Ok(())
        })
        .run();
}
