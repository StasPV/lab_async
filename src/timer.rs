use std::{future::Future, sync::{mpsc::{sync_channel, Receiver, SyncSender}, Arc, Mutex}, task::Waker, thread, time::Duration};
use std::pin::Pin;
use std::task::Poll;
use std::task::Context;

use futures::{future::BoxFuture, task::{waker_ref, ArcWake}};

pub struct TimerFuture{
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState{
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture{
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed{
            Poll::Ready(())
        }
        else{
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }

    }
}

impl TimerFuture{
    pub fn new(duration: Duration)-> Self{
        let shared_state = SharedState{
            completed: false,
            waker: None,
        };
        let shared_state = Arc::new(Mutex::new(shared_state));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move||{
            thread::sleep(duration);
             let mut shared_state = thread_shared_state.lock().unwrap();
             shared_state.completed = true;
             if let Some(waker) = shared_state.waker.take(){
                waker.wake();
             }
        });

        TimerFuture { shared_state }
    }
}

pub struct Executor{
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
pub struct Spawner{
    task_sender: SyncSender<Arc<Task>>,
}

struct Task{
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

pub fn new_executor_and_spawner()->(Executor, Spawner){
    const MAX_QUEUED_TASK: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASK);
    (Executor{ready_queue}, Spawner{task_sender})
}

impl Spawner{
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send){
        let future = Box::pin(future); // в оригинале здесь future.boxed() но такой метод не поддерживается
        let task = Task { future: Mutex::new(Some(future)), task_sender: self.task_sender.clone(), };
        let task = Arc::new(task);

        self.task_sender.try_send(task).expect("too many task queued");
    }
}

impl ArcWake for Task{
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.try_send(cloned).expect("too many task queued");
    }
}

impl Executor{
    pub fn run(&self){
        while let Ok(task) = self.ready_queue.recv(){
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take(){
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending(){
                    *future_slot = Some(future);
                }
            }
        }
    }
}