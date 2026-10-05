use lopdf::{Document, IncrementalDocument, Result};
use std::fmt::Display;
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};

#[allow(dead_code)]
pub fn load_document<P>(path: P) -> Result<Document>
where
    P: AsRef<Path> + Display,
{
    #[cfg(feature = "async")]
    let doc = tokio::runtime::Runtime::new()?.block_on(async move {
        Document::load(&path)
            .await
            .unwrap_or_else(|_| panic!("Failed to load {}", path))
    });
    #[cfg(not(feature = "async"))]
    let stop = Arc::new(AtomicBool::new(false));
    let doc = Document::load(path, stop)?;

    Ok(doc)
}

#[allow(dead_code)]
pub fn load_incremental_document<P>(path: P) -> Result<IncrementalDocument>
where
    P: AsRef<Path> + Display,
{
    #[cfg(feature = "async")]
    let doc = tokio::runtime::Runtime::new()?.block_on(async move {
        IncrementalDocument::load(&path)
            .await
            .unwrap_or_else(|_| panic!("Failed to load {}", path))
    });
    #[cfg(not(feature = "async"))]
    let stop = Arc::new(AtomicBool::new(false));
    let doc = IncrementalDocument::load(path, stop)?;

    Ok(doc)
}
