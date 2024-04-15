use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    /// Crea un Worker.
    /// id es el identificador del worker, se asigna por el loop en ThreadPool.
    /// receiver se encarga de la ejecucion de las tareas que van llegando por las requests
    /// # Panics
    ///
    /// En caso de error, el worker se desconecta
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
    /// Crea un ThreadPool.
    ///
    /// size es el número de threads que van a existir en el pool.
    ///
    /// # Panics
    ///
    /// La función no funcionará si size <= 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool {
            threads,
            sender: Some(sender),
        }
    }
    /// Crea la tarea y la envia por el canal del sender.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Apaga los workers y los une para que no queden tareas sin terminar.
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
