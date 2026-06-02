use std::{sync::mpsc, thread};

// import mutex 
use std::sync::{Arc, Mutex};



pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;



enum Message {
    NewJob(Job),
    Terminate,
}

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
        self.sender.send(Message::NewJob(job)).unwrap(); // send() returns a reuslt type 
        // send will fail if all our threads stop running 
    }
}



impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Sending terminate message to all workers");


        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        // we have no control over when the workers pick up our messages 
        // as long as num_workers = num_terminate messages, we should be fine 


        println!("Shutting down all workers");



        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}



struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        
        // Worker {id, thread}
        let thread = thread::spawn(move || loop {
            let message = receiver
            .lock()
            .unwrap()
            .recv()
            .unwrap();
        
            println!("Worker {id} got a job; executing.");
            // job();
            // receiver;
            match message  {
                Message::NewJob(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Message::Terminate => {
                    println!("Worker {id} was told to terminate.");
                    break;
                }
            }
        });



        Worker {id, thread: Some(thread)}
    }
}


// Besides the ThreadPool,
// we can also use the following methods to prevent requests from blocking each other
// 1. fork-join model
// 2. single-threaded async io model 

