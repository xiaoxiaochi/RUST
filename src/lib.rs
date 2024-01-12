use std::{thread::{JoinHandle, self}, process::id, sync::mpsc::{self, Receiver}};
use std::sync::{Arc,Mutex};
pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// #panics
    ///
    /// the new function will panic if the size is zero
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let (sender,receiver)=mpsc::channel();
        let receiver=Arc::new(Mutex::new(receiver));
        let mut workers=Vec::with_capacity(size);//这里不理解,就是获取线程池容量
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{workers,sender}
    }

    pub fn execute<F>(&self,f:F)
    where 
        F:FnOnce()+Send+'static
    {
        let job=Box::new(f);
        self.sender.send(job).unwrap();        
    }
}

trait FnBox {//这里将类型加Box，实现未知内存的解引用
    fn call_box(self:Box<Self>);
}

impl <F:FnOnce()> FnBox for F {//对FnOnce实现FnBox的方法重构，实现利用Box来进行解引用self
    fn call_box(self:Box<Self>) {
        (*self)()
    }
}
type Job=Box<dyn FnBox+Send+'static>;
struct Worker{
    id:usize,
    thread:thread::JoinHandle<()>,
}
impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread=thread::spawn(move || loop{

            let job=receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got Job",id);
            //(*job)();
            job.call_box();
        });
        Worker { id, thread }
    }
}

