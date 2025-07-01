use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub async fn watch_file(path: PathBuf, tx: mpsc::Sender<PathBuf>) -> anyhow::Result<()> {
    let (watcher_tx, watcher_rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(watcher_tx, Config::default())?;

    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    println!("Watching file: {}", path.display());

    tokio::spawn(async move {
        for res in watcher_rx {
            match res {
                Ok(event) => {
                    // We are only interested in file modifications
                    if event.kind.is_modify() {
                        if let Err(e) = tx.send(path.clone()).await {
                            eprintln!("Error sending file path: {e}");
                        }
                    }
                }
                Err(e) => eprintln!("watch error: {e:?}"),
            }
        }
    });

    Ok(())
}
