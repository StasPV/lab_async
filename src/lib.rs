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
use timer::TimerFuture;
use timer::new_executor_and_spawner;
pub fn timer_future(){
    let (execcutor, spawner) = new_executor_and_spawner();
    spawner.spawn(async{
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("Done!");
    });
    drop(spawner);

    execcutor.run();
}


#[cfg(test)]
mod lab_tests{
    #[test]
    fn dummy_test(){
        assert_eq!(1,1);
    }
}