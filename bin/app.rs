use crossbeam_channel;
use dotenv::dotenv;
use env_logger;
use log;
use met_io_rs::*;
use notify::{event::*, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dotenv().ok();
    env_logger::init();
    // let fname = r##"/mnt/d/BaiduNetdiskDownload/ANI_IR1_R04_20200509_0900_FY2G.AWX"##;
    // let fname = r##"/mnt/d/demo/KTDIA2018013112925012.grb"##;
    // let output = "/mnt/d/temp/grib";
    // let rt = Some(AWX);
    // let ot = vec![Diamond4,NOMGrid];
    // // convert_data(fname, output, None, Some(ot));
    // convert_data(fname, output, None, None);

    let config = get_config("config/monitor_config.json").unwrap();

    log::info!("{:?}", config);

    let source = &config.source;
    let dest = &config.destination;

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |res| tx.send(res).unwrap()).unwrap();

    let output = &dest.path;
    for s in source.iter() {
        watcher
            .watch(&Path::new(&s.path), RecursiveMode::Recursive)
            .unwrap();
    }

    for res in rx {
        match res {
            Ok(event) => {
                // println!("changed: {:?}", event);
                let kind = event.kind;
                if kind == EventKind::Access(AccessKind::Close(AccessMode::Write)) {
                    // if kind == EventKind::Create(CreateKind::File) {
                    log::info!("process files : {:?}", event.paths);
                    // std::thread::sleep(Duration::from_millis(200));
                    for p in event.paths.iter() {
                        convert_data(p.to_str().unwrap(), output, None, None);
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
#[cfg(target_arch = "wasm32")]
fn main(){}