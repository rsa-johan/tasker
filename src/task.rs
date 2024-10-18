use parry::Cmd;
use std::{env, fs, path::Path};

const LINE_SEPERATOR: &'static str = ";;";
const WORD_SEPERATOR: &'static str = "|";

#[derive(Debug, Default)]
struct Task {
    name: String,
    id: u32,
    time: Option<String>,
    date: Option<String>,
    status: Option<String>,
}

#[derive(Debug)]
pub struct Tasker {
    cmd: Cmd,
    command: String,
    path: String,
}

impl Tasker {
    pub fn new() -> Tasker {
        let mut path: String = String::new();
        if let Ok(p_) = env::var("TASK_SPACE") {
            path = p_;

            if !fs::exists(&path).unwrap() {
                fs::create_dir(&path)
                    .expect(format!("Unable to create a directory at {}", path).as_str());
            }

            path.push_str("\\main.tks");
        }

        Tasker {
            cmd: Cmd::new(),
            command: String::new(),
            path,
        }
    }

    pub fn setup(&mut self) -> &mut Self {
        self.cmd
            .arg("name".into(), Some("n".into()))
            .arg("time".into(), Some("t".into()))
            .arg("day".into(), Some("d".into()))
            .arg("status".into(), Some("s".into()))
            .parse();

        self.command = self.cmd.args.get(1).unwrap().to_string();
        println!("{:#?}", self.command);
        self
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        match self.command.as_str() {
            "add" => self.add(),
            "set" => self.set(),
            "list" => self.list(),
            _ => panic!("Unknown command!"),
        }
    }

    fn add(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.path);
        fs::File::create_new(path).unwrap();

        let mut content = fs::read_to_string(path)?;
        let mut task_list: Vec<Task> = Vec::new();

        if content.len() > 0 {
            task_list = content
                .split(LINE_SEPERATOR)
                .map(|x| Tasker::string_to_task(x))
                .collect();
        }

        let mut task = Task::default();

        self.get_task(&mut task);

        if task.name.len() < 1 {
            panic!("Name is Mandatory in case of Adding a task!!");
        }

        task.id = 1;
        if task_list.len() > 0 {
            task.id = task_list.get(task_list.len() - 1).unwrap().id + 1;
        }

        let line = Tasker::task_to_string(&task);

        content.push_str(&line);
        content.push_str(LINE_SEPERATOR);
        content.push_str();

        fs::write(path, content)
    }

    fn set(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.path);
        let mut task = Task::default();
        let mut content = String::new();

        self.get_task(&mut task);

        let mut task_list = self.get_task_list()?;
        println!("Before: {:#?}", task_list);
        task_list.iter_mut().for_each(|x| {
            if x.id == task.id {
                if task.name.len() > 0 {
                    x.name = task.name.clone();
                }
                x.status = task.status.to_owned();
                x.date = task.date.to_owned();
                x.time = task.time.to_owned();
            }
        });

        println!("After: {:#?}", task_list);

        task_list.iter().for_each(|x| {
            let s = Tasker::task_to_string(&x);
            content.push_str(s.as_str());
        });

        fs::write(path, content)?;

        Ok(())
    }

    fn list(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.path);
        let content = fs::read_to_string(path)?;
        let mut display = String::from("");

        let id_header = "id";
        let name_header = "name";
        let status_header = "status";
        let time_header = "time";
        let date_header = "date";

        for line in content.split(LINE_SEPERATOR) {
            let task = Tasker::string_to_task(line);
        }

        Ok(())
    }

    fn get_task_list(&self) -> Result<Vec<Task>, std::io::Error> {
        let path = Path::new(&self.path);
        fs::File::create_new(path).unwrap();

        let content = fs::read_to_string(path)?;
        let mut task_list: Vec<Task> = Vec::new();

        if content.len() > 0 {
            task_list = content
                .split(LINE_SEPERATOR)
                .map(|x| Tasker::string_to_task(x))
                .collect();
        }

        Ok(task_list)
    }

    fn get_task(&self, task: &mut Task) {
        if let Some(name) = self.cmd.get("name") {
            task.name = name.to_owned();
        }
        if let Some(id) = self.cmd.get("id") {
            task.id = id.parse().expect("ID should be a number!!");
        }

        task.status = self.cmd.get("status").cloned();
        task.date = self.cmd.get("date").cloned();
        task.time = self.cmd.get("time").cloned();
    }

    fn string_to_task(line: &str) -> Task {
        let mut task = Task::default();

        line.split(WORD_SEPERATOR).for_each(|x| {
            if x.starts_with("@") {
                task.id = x.parse().unwrap();
            } else if x.starts_with("d:") {
                task.date = Some(x.to_string());
            } else if x.starts_with("t:") {
                task.time = Some(x.to_string());
            } else if x.starts_with("s:") {
                task.status = Some(x.to_string());
            } else if x.starts_with("n:") {
                task.name = x.to_string();
            }
        });

        task
    }

    fn task_to_string(task: &Task) -> String {
        let mut line = format!("@{}|n:{}|", task.id, task.name);
        if let Some(val) = &task.date {
            line.push_str("d:");
            line.push_str(&val);
            line.push_str(WORD_SEPERATOR);
        }
        if let Some(val) = &task.time {
            line.push_str("t:");
            line.push_str(&val);
            line.push_str(WORD_SEPERATOR);
        }
        if let Some(val) = &task.status {
            line.push_str("s:");
            line.push_str(&val);
        } else {
            line.push_str("s:TODO");
        }
        line
    }
}
