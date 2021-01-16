mod fifo;
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Task {
   pub id: &'static str,
   pub duration:u32,
   pub deadline: u32,
}
pub trait SchedulingPolicy{
   fn register(&mut self, task:Task);
   fn take(&mut self) -> Option<Task>;
}
 
#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
       assert_eq!(2+2,4);
   }
}
