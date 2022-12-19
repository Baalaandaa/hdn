use chrono::Utc;

/// Prints entry log to stdout
pub fn log_entry(ip: String, storage_size: usize) {
    println!(
        "{} [{}] Connection established. Storage size: {}.",
        ip,
        Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        storage_size
    )
}

/// Prints log of store request to stdout
pub fn log_store(ip: String, key: &String, value: &String, storage_size: usize) {
    println!(
        "{} [{}] Received request to write new value `{}` by key `{}`. Storage size: {}.",
        ip,
        Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        value,
        key,
        storage_size
    )
}

/// Prints log of load request to stdout
pub fn log_load(ip: String, key: &String, storage_size: usize) {
    println!(
        "{} [{}] Received request to get value by key `{}`. Storage size: {}.",
        ip,
        Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        key,
        storage_size
    )
}
