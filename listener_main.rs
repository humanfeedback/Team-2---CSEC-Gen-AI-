use eframe::{egui, epi};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

struct TcpApp {
    received_data: Arc<Mutex<String>>,
}

impl Default for TcpApp {
    fn default() -> Self {
        Self {
            received_data: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl epi::App for TcpApp {
    fn name(&self) -> &str {
        "TCP Listener GUI"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TCP Listener on Port 4444");
            // Clone the data for display
            let data = self.received_data.lock().unwrap().clone();
            // Use a label to display the data, filling the entire window
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.label(data);
            });
        });
    }
}

fn set_non_blocking(stream: &TcpStream) {
    stream.set_nonblocking(true).expect("Failed to set to non-blocking");
}

fn main() {
    let app = TcpApp::default();
    let received_data = app.received_data.clone();

    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:4444").expect("Could not bind to port 4444");
        println!("Listening on port 4444...");
        listener.set_nonblocking(true).expect("Failed to set listener to non-blocking");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    set_non_blocking(&stream);
                    let mut buffer = [0; 1024];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(size) => {
                                if size > 0 {
                                    let msg = String::from_utf8_lossy(&buffer[..size]).to_string();
                                    let mut data = received_data.lock().unwrap();
                                    data.push_str(&msg);
                                    std::fs::write("log.txt", &*data).expect("Failed to write to log.txt");
                                }
                            },
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // Non-blocking call would block here, so we just yield the thread.
                                std::thread::sleep(Duration::from_millis(100));
                            },
                            Err(_) => break, // Handle other errors by breaking the loop.
                        }
                    }
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No incoming connection was ready to be accepted, just yield the thread.
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                },
                Err(e) => println!("Connection failed: {}", e),
            }
        }
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), options);
}
