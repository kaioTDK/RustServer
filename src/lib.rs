use std::{thread, sync::{mpsc::{self, Receiver}, Mutex, Arc}};

pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() +  Send + 'static>;

enum Message {
    Newjob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        
        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&self, f: F) 
    where
        F:FnOnce() + Send + 'static
    {

        let job = Box::new(f);
        self.sender.send(Message::Newjob(job)).unwrap();

    }
}

impl Drop for ThreadPool { 
    fn drop(&mut self) {

        for i in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||  loop{
            
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Newjob(job) => {
                    println!("Worker {} got a job; executing.",id);
            
                    job();
                }
                
                Message::Terminate => {break;}
            }
        });

        Worker {id,thread: Some(thread)}
    }
    
}