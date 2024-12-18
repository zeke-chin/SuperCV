use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

// 激活指定窗口
pub fn activate_window(conn: &RustConnection, screen: &Screen, target_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let children = conn.query_tree(screen.root)?.reply()?.children;
    let net_active_window = conn.intern_atom(false, b"_NET_ACTIVE_WINDOW")?.reply()?.atom;

    for child in children {
        let title = get_window_title(conn, child)?;
        if title.contains(target_name) {
            // 1. 获取窗口所在桌面
            let net_wm_desktop = conn.intern_atom(false, b"_NET_WM_DESKTOP")?.reply()?.atom;
            let desktop = get_window_desktop(conn, child)?;

            // 2. 如果需要，切换到窗口所在桌面
            if desktop >= 0 {
                let current_desktop = get_current_desktop(conn, screen)?;
                if current_desktop != desktop {
                    conn.send_event(
                        false,
                        screen.root,
                        EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                        ClientMessageEvent::new(32, screen.root, net_wm_desktop, [desktop as u32, 0, 0, 0, 0]),
                    )?;
                }
            }

            // 3. 发送激活窗口消息
            conn.send_event(
                false,
                screen.root,
                EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                ClientMessageEvent::new(32, child, net_active_window, [1, x11rb::CURRENT_TIME, 0, 0, 0]),
            )?;

            // 4. 确保窗口可见
            conn.map_window(child)?;

            // 5. 刷新连接
            conn.flush()?;
            return Ok(());
        }
    }

    println!("Window '{}' not found", target_name);
    Ok(())
}

// 获取当前工作区
fn get_current_desktop(conn: &RustConnection, screen: &Screen) -> Result<i32, Box<dyn std::error::Error>> {
    let net_current_desktop = conn.intern_atom(false, b"_NET_CURRENT_DESKTOP")?.reply()?.atom;

    let desktop = conn
        .get_property(false, screen.root, net_current_desktop, AtomEnum::CARDINAL, 0, 1)?
        .reply()?;

    if !desktop.value.is_empty() {
        Ok(i32::from_ne_bytes([
            desktop.value[0],
            desktop.value.get(1).copied().unwrap_or(0),
            desktop.value.get(2).copied().unwrap_or(0),
            desktop.value.get(3).copied().unwrap_or(0),
        ]))
    } else {
        Ok(0)
    }
}

// 获取窗口的类名
fn get_window_class(conn: &impl Connection, window: Window) -> Result<String, Box<dyn std::error::Error>> {
    let class = conn.get_property(false, window, AtomEnum::WM_CLASS, AtomEnum::STRING, 0, 1024)?.reply()?;

    if class.type_ == u32::from(AtomEnum::STRING) {
        Ok(String::from_utf8_lossy(&class.value).to_string())
    } else {
        Ok(String::new())
    }
}

// 获取窗口所在的桌面编号
fn get_window_desktop(conn: &impl Connection, window: Window) -> Result<i32, Box<dyn std::error::Error>> {
    let net_wm_desktop = conn.intern_atom(false, b"_NET_WM_DESKTOP")?.reply()?.atom;

    let desktop = conn.get_property(false, window, net_wm_desktop, AtomEnum::CARDINAL, 0, 1)?.reply()?;

    if !desktop.value.is_empty() {
        Ok(i32::from_ne_bytes([
            desktop.value[0],
            desktop.value.get(1).copied().unwrap_or(0),
            desktop.value.get(2).copied().unwrap_or(0),
            desktop.value.get(3).copied().unwrap_or(0),
        ]))
    } else {
        Ok(-1)
    }
}

// 获取窗口的标题
fn get_window_title(conn: &impl Connection, window: Window) -> Result<String, Box<dyn std::error::Error>> {
    let name = conn.get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024)?.reply()?;

    if name.type_ == u32::from(AtomEnum::STRING) {
        Ok(String::from_utf8_lossy(&name.value).to_string())
    } else {
        Ok(String::new())
    }
}
