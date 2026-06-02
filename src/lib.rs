use std::{sync::mpsc, thread};

// import mutex 
use std::sync::{Arc, Mutex};



pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    // compiler-driven development (similar to test-driven development)
    // 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // just check that size is greater than 0, and it would panic if that is not the case. 

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));


        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // the spawn function takes a closure which will be executed immediately after a thread is executed
            // which we don't want. 
            // we want to create threads  (that waits for some code to execute later on)
            // store another struct called workers (with an id and a thread that listens for some jobs to be done.)
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }




        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap(); // send() returns a reuslt type 
        // send will fail if all our threads stop running 
    }
}



struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        
        // Worker {id, thread}
        let thread = thread::spawn(move || {
            let job = receiver
            .lock()
            .unwrap()
            .recv()
            .unwrap();
        
            println!("Worker {id} got a job; executing.");
            job();
            // receiver;
        });

        Worker {id, thread}
    }
}


// Besides the ThreadPool,
// we can also use the following methods to prevent requests from blocking each other
// 1. fork-join model
// 2. single-threaded async io model 

