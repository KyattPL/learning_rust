pub struct Process {
    time_in_queue: i32,
    task_time: i32,
    time_processed: i32,
    no_of_process: i32,
}

impl Process {
    pub fn new(task_time: i32, no_of_process: i32) -> Process {
        Process {
            time_in_queue: 0,
            task_time,
            time_processed: 0,
            no_of_process,
        }
    }

    pub fn set_task_time(&mut self, time: i32) {
        self.task_time = time;
    }

    pub fn get_task_time(&self) -> i32 {
        self.task_time
    }

    pub fn add_time_in_queue(&mut self) {
        self.time_in_queue += 1;
    }

    pub fn get_time_in_queue(&self) -> i32 {
        self.time_in_queue
    }

    pub fn add_time_processed(&mut self) {
        self.time_processed += 1;
    }

    pub fn get_time_processed(&self) -> i32 {
        self.time_processed
    }

    pub fn get_no_of_process(&self) -> i32 {
        self.no_of_process
    }
}
