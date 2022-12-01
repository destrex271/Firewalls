use std::{
    sync::{
        mpsc,
        Arc,
        Mutex
    },
    thread
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing!");
            job();
        });
        Worker{
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool{
    pub fn build(n: usize) -> ThreadPool{
        assert!(n > 0);
        let mut pool = Vec::with_capacity(n);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..n{
            let worker = Worker::new(i+1, receiver.clone()); 
            pool.push(worker);
        }
        ThreadPool{
            workers: pool,
            sender
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for worker in &mut self.workers{
            println!("Shutting Down!");
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
