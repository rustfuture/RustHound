use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub fn watch_file(path: PathBuf, tx: mpsc::Sender<PathBuf>) -> anyhow::Result<RecommendedWatcher> {
    let callback_path = path.clone();
    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| match res {
            Ok(event) if event.kind.is_modify() => {
                // A full channel already signals a re-read from the saved offset, so extra signals can be dropped.
                let _ = tx.try_send(callback_path.clone());
            }
            Ok(_) => {}
            Err(e) => eprintln!("watch error: {e:?}"),
        },
        Config::default(),
    )?;

    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    println!("Watching file: {}", path.display());

    Ok(watcher)
}

#[cfg(test)]
mod tests {
    use super::watch_file;
    use std::{fs, path::PathBuf, time::Duration};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn watch_file_reports_appended_line() {
        let path = std::env::temp_dir().join(format!(
            "rusthound-watch-file-{}-{}.log",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::write(&path, "initial\n").unwrap();
        let (tx, mut rx) = mpsc::channel(1);

        let watcher = watch_file(path.clone(), tx).unwrap();
        fs::write(&path, "initial\nappended\n").unwrap();

        let received = tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("timed out waiting for file modification")
            .expect("watch channel closed");
        assert_eq!(received, PathBuf::from(&path));
        drop(watcher);
        fs::remove_file(path).unwrap();
    }
}
