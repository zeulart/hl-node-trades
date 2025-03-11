mod config;

use notify::{RecursiveMode, RecommendedWatcher, Watcher, EventKind};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use linemux::MuxedLines;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;
    let watched_files = Arc::new(RwLock::new(HashSet::new()));
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        notify::Config::default(),
    )?;
    watcher.watch(Path::new(&config.root_dir), RecursiveMode::Recursive)?;

    while let Some(event) = rx.recv().await {
        if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
            for path in event.paths {
                if !path.is_file() {
                    continue;
                }

                let watched_files = watched_files.clone();
                let mut watched = watched_files.write().await; // Use write lock to modify

                if watched.contains(&path) {
                    continue;
                }
                watched.insert(path.clone()); // Add file before spawning task
                tokio::spawn(async move {
                    println!("watching: {}", path.display());

                    let mut lines = match MuxedLines::new() {
                        Ok(lines) => lines,
                        _ => return,
                    };

                    if let Err(e) = lines.add_file_from_start(&path).await {
                        return;
                    }

                    while let Ok(Some(line)) = lines.next_line().await {
                        println!("source: {}, line: {}", line.source().display(), line.line());
                    }
                });
            }
        }
    }

    Ok(())
}