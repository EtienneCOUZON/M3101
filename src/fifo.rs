struct FifoScheduler{
    pub fn new(){
        impl SchedulingPolicy for FifoScheduler
    }
}
[cfg(test)]
mod tests {
    use super::FifoScheduler;
    use super::super::{SchedulingPolicy, Task};

    #[test]
    fn test_fifo(){
        
        assert_eq!(2 + 2, 4);
        
    }
}