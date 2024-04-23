use std::{
    sync::Arc,
    sync::Mutex,
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    // 에러 발생 코드
    //let mut data = 100;

    //thread::spawn(|| {
    //    data = 500;
    //});
    //thread::spawn(|| {
    //    data = 1_000;
    //});

    let m_data = Arc::new(Mutex::new(100));
    let mut th_vec: Vec<JoinHandle<()>> = vec![];

    let first_data = Arc::clone(&m_data);

    th_vec.push(thread::spawn(move || {
        let mut in_data = first_data.lock().unwrap();

        *in_data = 500;

        thread::sleep(Duration::from_millis(9));
    }));

    let second_data = Arc::clone(&m_data);

    th_vec.push(thread::spawn(move || {
        let mut in_data = second_data.lock().unwrap();

        *in_data = 1_000;

        thread::sleep(Duration::from_millis(12));
    }));

    for h in th_vec {
        h.join().unwrap();
    }

    println!("final data : {}", m_data.lock().unwrap());
}
