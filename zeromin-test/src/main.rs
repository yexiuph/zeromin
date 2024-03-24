#[allow(warnings)]
use lazy_static::lazy_static;
#[allow(unused)]
use std::{
    path,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use tokio::time::Instant;
use zeromin_common::utils::{
    subpath::SUBPATH,
    unzipper::{CUnzipper, FileStream},
};

lazy_static! {
    static ref RCCFILE: Arc<Mutex<CUnzipper>> = Arc::new(Mutex::new(CUnzipper::new()));
    static ref PATHS: SUBPATH = SUBPATH::new(); // Declare PATHS as static
}

#[allow(unused)]
pub async fn all_loaded_files(rccfile: &CUnzipper) -> Vec<FileStream> {
    rccfile.files.lock().unwrap().clone()
}

#[allow(unused)]
pub async fn load_rcc_file(rccfile: Arc<Mutex<CUnzipper>>, path: &str) -> Instant {
    let start_time = Instant::now();
    let rccfile = rccfile.lock().unwrap();

    if let Err(err) = rccfile.load_rcc(path) {
        eprintln!("Error: {:?}", err);
    } else {
        println!("{:?} : Extraction successful!", path);
        for file in rccfile.files.lock().unwrap().iter() {
            //println!("File Name: {:?}", file.file_name);
            //println!("File Bytes: {:?}", file.file_content);

            //? I've added sleep for human eyes to be able to see how it is processing the bytes - YeXiuPH
            //? Remove when demoing how fast it would load.
            //sleep(Duration::from_secs(5));
        }
    }
    println!("Loaded {} in {:?}",path, start_time.elapsed());
    start_time

}

#[allow(unused)]
pub async fn file_data(rccfile: &CUnzipper, file_name: &str) -> Option<FileStream> {
    // Find the file with the specified name in the ZIP archive
    rccfile
        .files
        .lock()
        .unwrap()
        .iter()
        .find(|file| file.file_name == file_name)
        .cloned()
}

#[allow(unused)]
#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let rccfile = RCCFILE.clone();

    //? Load RCC Files of RAN Online - YeXiuPH
    let glogic_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.glogic));
    let level_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.glevel));
    let npctalk_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.gnpctalk));
    let quest_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.gquest));
    let effect_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.geffect));
    let chareffect_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.gchareffect));
    let animation_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.ganimation));
    let skinobject_data = tokio::task::spawn(load_rcc_file(rccfile.clone(), &PATHS.gskinobject));

    let result = tokio::try_join!(
        glogic_data,
        level_data,
        npctalk_data,
        quest_data,
        effect_data,
        chareffect_data,
        animation_data,
        skinobject_data
    );

    match result {
        Ok((glogic, level, npctalk, quest, effect, chareffect, animation, skinobject)) => {
            let total_elapsed_time = glogic.elapsed() + level.elapsed() + npctalk.elapsed() + quest.elapsed() + effect.elapsed() + chareffect.elapsed() + animation.elapsed() + skinobject.elapsed();
            println!("All tasks completed successfully!");
            println!("Total Time : {:?}", total_elapsed_time)
        }
        Err(err) => {
            eprintln!("Error in one of the tasks: {:?}", err);
        }
    }

    // Access the loaded files and find a specific file by name
    let rccfile_guard = rccfile.lock().unwrap();
    if let Some(requested_file) = file_data(&*rccfile_guard, "Archer.charset").await {
        println!("Found desired file: {:?}", requested_file.file_name);
        //println!("File Bytes: {:?}", requested_file.file_content);
    }

    if let Some(requested_file) = file_data(&*rccfile_guard, "w_school_04_in_b3.lev").await {
        println!("Found desired file: {:?}", requested_file.file_name);
        //println!("File Bytes: {:?}", requested_file.file_content);
    }

    if let Some(requested_file) = file_data(&*rccfile_guard, "a_m_03_strock.cfg").await {
        println!("Found desired file: {:?}", requested_file.file_name);
        //println!("File Bytes: {:?}", requested_file.file_content);
    }

    let loaded_files = all_loaded_files(&*rccfile_guard).await;
    
    for file in loaded_files {
        println!("Loaded : {}", file.file_name)
    }
}
