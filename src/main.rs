// Macro này sẽ tự động kết nối với file .slint bạn đã tạo
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 1. Khởi tạo cửa sổ ứng dụng
    let ui = AppWindow::new()?;

    // 2. Tạo một 'con trỏ yếu' (weak pointer) để dùng an toàn trong các callback
    let ui_handle = ui.as_weak();

    // 3. Xử lý sự kiện khi bấm nút TĂNG SỐ
    ui.on_request_increase({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let current_value = ui.get_counter();
            ui.set_counter(current_value + 1);
            println!("Giá trị hiện tại: {}", current_value + 1);
        }
    });

    // 4. Xử lý sự kiện khi bấm nút RESET
    ui.on_request_reset(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(0);
        println!("Đã reset bộ đếm về 0");
    });

    // 5. Chạy ứng dụng (Event Loop)
    ui.run()
}