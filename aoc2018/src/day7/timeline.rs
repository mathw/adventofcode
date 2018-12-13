use super::task::Task;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ThreadTask {
    Empty(u32),
    Busy(Task),
}

impl ThreadTask {
    fn duration(&self) -> u32 {
        match self {
            ThreadTask::Empty(time) => *time,
            ThreadTask::Busy(task) => task.duration(),
        }
    }

    fn name(&self) -> Option<char> {
        match self {
            ThreadTask::Busy(task) => Some(task.name()),
            _ => None,
        }
    }
}

struct Thread {
    tasks: Vec<ThreadTask>,
}

impl Thread {
    fn new() -> Thread {
        Thread { tasks: Vec::new() }
    }

    fn end_time(&self) -> u32 {
        self.tasks.iter().fold(0, |acc, task| acc + task.duration())
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(ThreadTask::Busy(task))
    }

    fn task_at(&self, time: u32) -> Option<ThreadTask> {
        let mut current_time = 0;
        let mut current_task = None;
        for task in self.tasks.iter() {
            current_time += task.duration() - 1;
            current_task = Some(task.clone());
            if time <= current_time {
                return current_task;
            }
        }

        if current_time < time {
            return None;
        }

        return current_task;
    }

    fn pad_to(&mut self, time: u32) {
        let end_time = self.end_time();

        if end_time < time {
            self.tasks.push(ThreadTask::Empty(time - end_time));
        }
    }

    fn tasks_up_to(&self, time: u32) -> Vec<ThreadTask> {
        let mut current_time = 0;
        let mut completed = Vec::new();
        for task in self.tasks.iter() {
            current_time += task.duration();
            if time >= current_time {
                completed.push(task.clone());
            } else {
                break;
            }
        }
        completed
    }
}

pub struct Timeline {
    threads: Vec<Thread>,
}

impl Timeline {
    pub fn new(workers: usize) -> Timeline {
        let mut threads = Vec::with_capacity(workers);
        for _ in 0..workers {
            threads.push(Thread::new())
        }

        Timeline { threads }
    }

    pub fn add_task(&mut self, task: Task) {
        let thread = self
            .threads
            .iter()
            .enumerate()
            .map(|(thread_num, thread)| (thread_num, thread.end_time()))
            .min_by_key(|(_, t)| t.clone())
            .map(|(thread, _)| thread)
            .unwrap_or(0);

        self.threads[thread].add_task(task);
    }

    pub fn add_task_after(&mut self, time: u32, task: Task) {
        for thread in self.threads.iter_mut() {
            thread.pad_to(time);
        }

        self.add_task(task);
    }

    pub fn total_time_required(&self) -> u32 {
        self.threads
            .iter()
            .map(|thread| thread.end_time())
            .max()
            .unwrap_or(0)
    }

    pub fn completed_at(&self, time: u32) -> HashSet<char> {
        self.threads
            .iter()
            .map(|thread| thread.tasks_up_to(time))
            .flatten()
            .filter_map(|task| task.name())
            .collect()
    }

    pub fn free_workers_at(&self, time: u32) -> usize {
        let tasks_at = self
            .threads
            .iter()
            .map(|t| t.task_at(time))
            .filter(|t| match t {
                Some(ThreadTask::Busy(_)) => true,
                _ => false,
            })
            .count();

        self.threads.len() - tasks_at
    }
}

impl Display for Timeline {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "Timeline  ({} threads)", self.threads.len())?;

        for time in 0..self.total_time_required() {
            write!(fmt, "{:02}:", time)?;

            for thread in self.threads.iter() {
                let t = thread.task_at(time);
                match t {
                    Some(ThreadTask::Busy(task)) => write!(fmt, " {} ", task.name()),
                    Some(ThreadTask::Empty(_)) => write!(fmt, " . "),
                    None => write!(fmt, " ? "),
                }?;
            }

            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

#[test]
fn thread_task_at() {
    let mut thread = Thread::new();

    assert_eq!(thread.task_at(0), None);
    assert_eq!(thread.task_at(2), None);
    assert_eq!(thread.task_at(7), None);

    thread.add_task(Task::new('A', 3));
    assert_eq!(thread.task_at(0).is_some(), true);
    assert_eq!(thread.task_at(2).is_some(), true);
    assert_eq!(
        thread.task_at(3).is_some(),
        false,
        "End of task should not show the task!"
    );
}

#[test]
fn thread_pad_to() {
    let mut thread = Thread::new();

    assert_eq!(thread.end_time(), 0, "before padding");

    thread.pad_to(4);

    assert_eq!(thread.end_time(), 4, "after padding");

    thread.add_task(Task::new('A', 2));

    assert_eq!(thread.end_time(), 6, "after task");
}

#[test]
fn timeline_add_after() {
    let mut timeline = Timeline::new(2);

    let task = Task::new('A', 2);

    timeline.add_task_after(2, task);

    println!("{}", timeline);
    assert_eq!(timeline.total_time_required(), 4);
}

#[test]
fn example_tasks_in_order() {
    let mut timeline = Timeline::new(2);
    let task_a = Task::new('A', 1);
    let task_b = Task::new('B', 2);
    let task_c = Task::new('C', 3);
    let task_d = Task::new('D', 4);
    let task_e = Task::new('E', 5);
    let task_f = Task::new('F', 6);

    timeline.add_task(task_c.clone());
    assert_eq!(timeline.total_time_required(), 3);

    timeline.add_task_after(task_c.duration(), task_a.clone());
    timeline.add_task_after(task_c.duration(), task_f);
    timeline.add_task_after(task_c.duration() + task_a.duration(), task_b.clone());
    timeline.add_task_after(
        task_c.duration() + task_a.duration() + task_b.duration(),
        task_d.clone(),
    );
    timeline.add_task_after(
        task_c.duration() + task_a.duration() + task_b.duration() + task_d.duration(),
        task_e,
    );

    println!("{}", timeline);

    assert_eq!(timeline.total_time_required(), 15);
}

#[test]
fn free_workers_at() {
    let mut timeline = Timeline::new(3);

    timeline.add_task(Task::new('A', 2));
    timeline.add_task(Task::new('B', 3));

    assert_eq!(
        timeline.free_workers_at(0),
        1,
        "At the start there should be one free worker"
    );
    assert_eq!(
        timeline.free_workers_at(1),
        1,
        "At 1 there should be one free worker"
    );
    assert_eq!(
        timeline.free_workers_at(2),
        2,
        "At 2 there should be two free workers because A has finished"
    );
    assert_eq!(
        timeline.free_workers_at(3),
        3,
        "At 3 there should be three free workers because B has finished"
    );
}

#[test]
fn timeline_completed_at() {
    let mut timeline = Timeline::new(2);
    timeline.add_task(Task::new('A', 1));
    timeline.add_task(Task::new('B', 2));
    timeline.add_task(Task::new('C', 2));

    assert_eq!(timeline.completed_at(0).len(), 0, "Nothing completed at 0");
    assert_eq!(
        timeline.completed_at(1).len(),
        1,
        "One thing completed at 1"
    );
    assert_eq!(
        timeline.completed_at(2).len(),
        2,
        "Two things completed at 2"
    );
    assert_eq!(
        timeline.completed_at(3).len(),
        3,
        "Three things completed at 3"
    );
}
