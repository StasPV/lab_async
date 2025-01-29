use std::time::Duration;

use futures::executor::block_on;

pub fn future_hello_world() {
    let future = hello_world();
    block_on(future);
}

async fn hello_world() {
    println!("Hello, world!");
}

mod timer;
use timer::new_executor_and_spawner;
use timer::TimerFuture;
pub fn timer_future() {
    let (execcutor, spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("Done!");
    });
    drop(spawner);

    execcutor.run();
}

mod pin;
use pin::Test;
pub fn pin_lab() {
    let test1 = Test::new("test1");
    // let mut test1 = unsafe {
    //     Pin::new_unchecked(&mut test1)
    // };
    // Test::init(test1.as_mut());

    let test2 = Test::new("test2");
    // let mut test2 = unsafe {
    //     Pin::new_unchecked(&mut test2)
    // };
    // Test::init(test2.as_mut());

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
    // std::mem::swap(test1.get_mut(),  test2.get_mut());
}


#[cfg(test)]
#[warn(unused_imports)]
mod lab_tests {
    use crate::*;
    
    #[test]
    fn dummy_test() {
        println!("dummy_test начался");
        assert_eq!(1, 1);
    }
    
}
