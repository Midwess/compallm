use compallm_core::compression::DCompression;

pub fn compress(codec: &DCompression, value: &str) -> anyhow::Result<String> {
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?
                    .block_on(codec.compress(value))
            })
            .join()
            .map_err(|_| anyhow::anyhow!("compression test thread failed"))?
    })
}

pub fn decompress(codec: &DCompression, value: &str) -> anyhow::Result<String> {
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?
                    .block_on(codec.decompress(value))
            })
            .join()
            .map_err(|_| anyhow::anyhow!("decompression test thread failed"))?
    })
}
