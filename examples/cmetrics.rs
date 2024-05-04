use anyhow::Result;
use concurrency::CmapMetrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = CmapMetrics::new();
    println!("{}", metrics);
    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }
    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    loop {
        thread::sleep(Duration::from_secs(10));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, mut metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_secs(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(mut metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
