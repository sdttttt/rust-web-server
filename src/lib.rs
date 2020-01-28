pub mod MultiThreading {

    use std::thread;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::mpsc;


    pub struct ThreadPool {
        workers: Vec<Worker>,
        // Job is a Function
        sender: mpsc::Sender<Job>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    struct Worker {
        id: usize,
        // JoinHandle is a Thread
        thread: thread::JoinHandle<()>,
    }

    impl ThreadPool {
        pub fn new(count: usize) -> Self {
            assert!(count > 0);

            let (sender, receiver) = mpsc::channel();
            let mut threads = Vec::with_capacity(count);

            let receiver = Arc::new(Mutex::new(receiver));

            for num in 0..count {
                threads.push(Worker::new(num, Arc::clone(&receiver)));
            }

            ThreadPool {
                workers: threads,
                sender,
            }
        }

        pub fn execute<F>(&self, f: F)
            where F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Self {
            let thread = thread::spawn(move|| {
                loop {
                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("Worker {} got a job! Execute!", id);
                    job();
                }
            });

            Worker {
                id,
                thread,
            }
        }
    }

}