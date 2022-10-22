use std::thread;
use std::thread::Builder;
use std::io::Error;
use std::sync::{mpsc, mpsc::SendError, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker
{
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, Error>
    {
        let thread = Builder::new().spawn
        (
            move || loop
            {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            }
        )?;

        Ok(
            Worker
            {
                id,
                thread
            }
        )
    }
}

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool
{
    pub fn new(size: usize) -> Result<ThreadPool, Error>
    {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))?);
        }

        Ok(ThreadPool {workers, sender})
    }

    pub fn execute<F>(&self, f: F) -> Result<(), SendError<Job>>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job)?;
        
        Ok(())
    }
}