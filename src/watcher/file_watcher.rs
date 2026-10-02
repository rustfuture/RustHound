use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub async fn watch_file(path: PathBuf, tx: mpsc::Sender<PathBuf>) -> anyhow::Result<()> {
    let (watcher_tx, watcher_rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(watcher_tx, Config::default())?;

    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    println!("Watching file: {}", path.display());

    tokio::task::spawn_blocking(move || {
        // Keep the OS watcher alive for as long as we consume its events.
        let _watcher = watcher;
        for res in watcher_rx {
            match res {
                Ok(event) => {
                    // We are only interested in file modifications
                    if event.kind.is_modify() {
                        if let Err(e) = tx.blocking_send(path.clone()) {
                            eprintln!("Error sending file path: {e}");
                            break;
                        }
                    }
                }
                Err(e) => eprintln!("watch error: {e:?}"),
            }
        }
    });

    Ok(())
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

        watch_file(path.clone(), tx).await.unwrap();
        fs::write(&path, "initial\nappended\n").unwrap();

        let received = tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("timed out waiting for file modification")
            .expect("watch channel closed");
        assert_eq!(received, PathBuf::from(&path));
        fs::remove_file(path).unwrap();
    }
}
