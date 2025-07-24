use tokio::sync::mpsc;
use cloud_notepad::{NoteApp, CloudService};

#[tokio::main]
async fn main() -> eframe::Result<()> {
    // 初始化通道
    let (message_sender, message_receiver) = mpsc::channel(21);
    let (response_sender, response_receiver) = mpsc::channel(21);

    // 启动认证服务
    let base_url = "http://10.124.200.172:8080/api";
    let service = CloudService::new(base_url, response_sender).expect("Failed to create service");
    tokio::spawn(async move {
        service.run(message_receiver).await;
    });

    // 初始化UI应用
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cloud Notepad", 
        options, 
        Box::new(|cc| Ok(Box::new(NoteApp::new(cc, message_sender, response_receiver)?))),
    )
}
