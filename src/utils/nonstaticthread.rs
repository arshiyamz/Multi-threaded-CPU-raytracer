use std::thread;
use std::thread::Builder;
use std::io::Error;
use std::mem;

pub struct NonStaticBuilder
{
    builder: Builder,
    handles: Vec<thread::JoinHandle<()>>,
}

impl<'a> NonStaticBuilder
{
    pub fn spawn<F: FnOnce() + Send + 'a>(mut self, f: F) -> Result<(), Error>
    {
        let func: Box<dyn FnOnce() + Send + 'a> = Box::new(f);
        let func: Box<dyn FnOnce() + Send + 'static> = unsafe { mem::transmute(func) };
        self.handles.push(self.builder.spawn(func)?);
        Ok(())
    }
}

//impl Drop for NonStaticBuilder
//{
//    fn drop(&mut self)
//    {
//        for handle in &self.handles
//        {
//            handle.join();
//        }
//    }
//}