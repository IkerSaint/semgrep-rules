use std::sync::RwLock;

fn no_deadlock() {
    let map: RwLock<Option<u32>> = RwLock::new(Some(2));

    if let Some(num) = *map.read().unwrap() {
        println!("There is a number over here {}", num);
    } else {
        let mut lock2 = map.write().unwrap();
        *lock2 = Some(5);
    }

    println!("Finish")
}

fn deadlock() {
    let my_lock: RwLock<Option<u32>> = RwLock::new(None);

    if let Some(data) = *my_lock.read().unwrap() {
        println!("Unwraped option: {:?}", data);
    } else {
        let mut data = data.write().unwrap();
        *data = Some(10);
        println!("Updated value {:?}", data);
    }

    println!("Goodbye!!");
}

fn main() {
    no_deadlock();
    deadlock();
}