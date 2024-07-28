use flexi_logger::{
    Cleanup, colored_opt_format, Criterion, Duplicate, FileSpec, Logger, Naming, opt_format,
};

use crate::utils::config::CONFIG;

pub fn init_logger(log_level: Option<i32>, sql_level: Option<i32>) {
    let logger_str = format!(
        "{}, sqlx={}",
        convert_log(log_level),
        convert_log(sql_level)
    );
    println!("logger setting: {}", logger_str);
    Logger::try_with_str(logger_str)
        .unwrap()
        .log_to_file(
            FileSpec::default().directory(CONFIG.read().unwrap().logs_path.to_str().unwrap()),
        )
        .format_for_files(opt_format)
        .format_for_stderr(colored_opt_format)
        .rotate(
            Criterion::Size(10 * 1024 * 1024), // 按大小切分，10MB
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10), // 保留10个日志文件
        )
        .duplicate_to_stderr(Duplicate::All)
        .start()
        .unwrap_or_else(|e| panic!("Logger init失败 err: {:?}", e));
}

pub fn convert_log(log_int: Option<i32>) -> String {
    match log_int {
        Some(0) => "trace".to_string(),
        Some(1) => "error".to_string(),
        Some(2) => "warn".to_string(),
        Some(3) => "info".to_string(),
        Some(4) => "debug".to_string(),
        Some(5) => "trace".to_string(),
        _ => "debug".to_string(),
    }
}

#[macro_export]
macro_rules! time_it {
    // 同步版本
    // let result = time_it!(sync { sync_function() })();
    (sync $func:expr) => {{
        let start = std::time::Instant::now();
        let result = $func;
        let duration = start.elapsed();
        time_it!(@log_duration, duration);
        result
    }};

    // 异步版本
    // let result = time_it!(async { add_clipboard_entry(&db, content) }).await;
    (async $($body:tt)*) => {{
        async {
            let start = tokio::time::Instant::now();
            let result = { $($body)* }.await;
            let duration = start.elapsed();
            time_it!(@log_duration, duration);
            result
        }
    }};

    // 内部用于记录日志的辅助宏
    (@log_duration, $duration:expr) => {{
        let nanos = $duration.as_nanos();
        let (value, unit) = if nanos >= 1_000_000_000 {
            (nanos as f64 / 1_000_000_000.0, "s")
        } else if nanos >= 1_000_000 {
            (nanos as f64 / 1_000_000.0, "ms")
        } else if nanos >= 1_000 {
            (nanos as f64 / 1_000.0, "µs")
        } else {
            (nanos as f64, "ns")
        };
        debug!(
            "execution time={:.3} {}",
            value, unit
        );
    }};
}
