use cloud_notepad::NoteApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cloud Notepad", 
        options, 
        Box::new(|cc| Ok(Box::new(NoteApp::new(cc)?))),
    )
}
