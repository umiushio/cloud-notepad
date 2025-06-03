use cloud_notepad::app::NoteApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cloud Notepad", 
        options, 
        Box::new(|cc| Ok(Box::new(NoteApp::new(cc)))),
    )
}
