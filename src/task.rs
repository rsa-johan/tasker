use parry::Cmd;
use std::{env, fs, path::Path, u16};

const LINE_SEPERATOR: &'static str = ";;";
const WORD_SEPERATOR: &'static str = "|";

#[derive(Debug)]
struct Meta {
    id: u16,
    name: u16,
    time: u16,
    date: u16,
    status: u16,
}

impl Meta {
    fn new() -> Meta {
        return Meta {
            id: "id".len() as u16,
            name: "name".len() as u16,
            time: "time".len() as u16,
            date: "date".len() as u16,
            status: "status".len() as u16,
        };
    }
}

#[derive(Debug, Default)]
struct Task {
    name: String,
    id: u16,
    time: Option<String>,
    date: Option<String>,
    status: Option<String>,
}

#[derive(Debug)]
pub struct Tasker {
    cmd: Cmd,
    command: String,
    path: String,
    meta_path: String,
}

impl Tasker {
    pub fn new() -> Tasker {
        let mut path: String = String::new();
        if let Ok(p_) = env::var("TASK_SPACE") {
            path = p_;
            if !Path::new(&path).exists() {
                fs::create_dir(&path)
                    .expect(format!("Unable to create a directory at {}", path).as_str());
            }

            path.push_str("\\main.tks");
            let path_1 = Path::new(&path);
            let _ = fs::File::create_new(path_1);
        }
        let meta_path_1 = Path::new(&path).parent().unwrap().join("meta.tkconf");
        let _ = fs::File::create_new(&meta_path_1);

        let mut meta_path = env::var("TASK_SPACE").unwrap();
        meta_path.push_str("\\meta.tkconf");

        Tasker {
            cmd: Cmd::new(),
            command: String::new(),
            path,
            meta_path,
        }
    }

    pub fn setup(&mut self) -> &mut Self {
        self.cmd
            .arg("name".into(), Some("n".into()))
            .arg("time".into(), Some("t".into()))
            .arg("date".into(), Some("d".into()))
            .arg("status".into(), Some("s".into()))
            .arg("id".into(), Some("i".into()))
            .arg("all".into(), None)
            .parse();

        self.command = self.cmd.args.get(1).unwrap().to_string();
        self
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        match self.command.as_str() {
            "add" => self.add(),
            "set" => self.set(),
            "list" => self.list(),
            "clear" => self.clear(),
            _ => panic!("Unknown command!"),
        }
    }

    fn add(&self) -> Result<(), std::io::Error> {
        let mut content = fs::read_to_string(&self.path)?;
        let mut task_list: Vec<Task> = Vec::new();

        if content.len() > 0 {
            task_list = content
                .split(LINE_SEPERATOR)
                .filter(|x| x.len() > 0)
                .map(|x| Tasker::string_to_task(x))
                .collect();
        } else {
            self.write_meta(&Meta::new())?;
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

        let mut meta = self.read_meta()?;
        Tasker::task_to_meta(&task, &mut meta);
        self.write_meta(&meta)?;

        fs::write(&self.path, content)
    }

    fn set(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.path);
        let mut task = Task::default();
        let mut content = String::new();

        self.get_task(&mut task);
        let mut meta = self.read_meta()?;

        let mut task_list = self.get_task_list()?;
        task_list.iter_mut().for_each(|x| {
            if x.id == task.id {
                if task.name.len() > 0 {
                    x.name = task.name.clone();
                }
                x.status = task.status.to_owned();
                x.date = task.date.to_owned();
                x.time = task.time.to_owned();
                let cmp_meta = Tasker::task_vs_task_to_meta(&task, x);
                Tasker::meta_vs_meta_to_meta(&mut meta, &cmp_meta);
            }
        });

        self.write_meta(&meta)?;

        task_list.iter().for_each(|x| {
            let s = Tasker::task_to_string(&x);
            content.push_str(s.as_str());
            content.push_str(LINE_SEPERATOR);
        });

        fs::write(path, content)?;

        Ok(())
    }

    fn list(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.path);
        let content = fs::read_to_string(path)?;
        if content.len() == 0 {
            println!("No task added");
            return Ok(());
        }
        let meta = self.read_meta()?;
        let id_header = Tasker::format("id", meta.id);
        let name_header = Tasker::format("name", meta.name);
        let status_header = Tasker::format("status", meta.status);
        let time_header = Tasker::format("time", meta.time);
        let date_header = Tasker::format("date", meta.date);

        println!(
            "{}{}{}{}{}|",
            id_header, name_header, date_header, time_header, status_header
        );

        let tasks: Vec<Task> = content
            .split(LINE_SEPERATOR)
            .filter(|x| x.len() > 1)
            .map(|x| Tasker::string_to_task(&x))
            .collect();

        for task in tasks {
            let id = Tasker::format(task.id, meta.id);
            let name = Tasker::format(task.name, meta.name);
            let status = Tasker::format(
                if let Some(v_) = task.status {
                    v_
                } else {
                    " ".to_string()
                },
                meta.status,
            );
            let time = Tasker::format(
                if let Some(v_) = task.time {
                    v_
                } else {
                    " ".to_string()
                },
                meta.time,
            );
            let date = Tasker::format(
                if let Some(v_) = task.date {
                    v_
                } else {
                    " ".to_string()
                },
                meta.date,
            );
            println!("{}{}{}{}{}|", id, name, date, time, status);
        }

        Ok(())
    }

    fn clear(&self) -> Result<(), std::io::Error> {
        let fcontent = fs::read_to_string(&self.path)?;
        let mut content = String::new();
        let mut tasks: Vec<Task> = fcontent
            .split(LINE_SEPERATOR)
            .filter(|x| x.len() > 1)
            .map(|x| Tasker::string_to_task(&x))
            .collect();

        if let Some(_) = self.cmd.get("all") {
            tasks.clear();
            fs::write(&self.path, "")?;
            self.write_meta(&Meta::new())?;
            return Ok(());
        }

        tasks.iter().for_each(|x| {
            if x.id == self.cmd.get("id").unwrap().parse().unwrap() {
                return;
            }
            let s = Tasker::task_to_string(&x);
            content.push_str(s.as_str());
            content.push_str(LINE_SEPERATOR);
        });

        fs::write(&self.path, content)
    }

    fn read_meta(&self) -> Result<Meta, std::io::Error> {
        let content = fs::read_to_string(&self.meta_path)?;
        Ok(Tasker::string_to_meta(content))
    }

    fn write_meta(&self, meta: &Meta) -> Result<(), std::io::Error> {
        let content = Tasker::meta_to_string(meta);
        fs::write(&self.meta_path, content)?;
        Ok(())
    }

    fn get_task_list(&self) -> Result<Vec<Task>, std::io::Error> {
        let path = Path::new(&self.path);
        let _ = fs::File::create_new(path);

        let content = fs::read_to_string(path)?;
        let mut task_list: Vec<Task> = Vec::new();

        if content.len() > 0 {
            task_list = content
                .split(LINE_SEPERATOR)
                .filter(|x| x.len() > 1)
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

    fn format<T: std::fmt::Display + std::fmt::Debug>(n: T, l: u16) -> String {
        format!("|{n:^l$}", n = n, l = l as usize)
    }

    fn task_vs_task_to_meta(task: &Task, a_task: &Task) -> Meta {
        let mut meta = Meta::new();

        let tid_1 = task.id.checked_ilog10().unwrap() as u16 + 1u16;
        let tid_2 = a_task.id.checked_ilog10().unwrap() as u16 + 1u16;

        meta.id = if tid_1 >= tid_2 { tid_1 } else { tid_2 };

        meta.name = if task.name.len() >= a_task.name.len() {
            task.name.len() as u16
        } else {
            a_task.name.len() as u16
        };

        meta.date = if let Some(s_) = &task.date {
            if let Some(a_s_) = &a_task.date {
                if s_.len() as u16 >= a_s_.len() as u16 {
                    s_.len() as u16
                } else {
                    a_s_.len() as u16
                }
            } else {
                s_.len() as u16
            }
        } else if let Some(a_s_) = &a_task.date {
            a_s_.len() as u16
        } else {
            0u16
        };

        meta.time = if let Some(s_) = &task.time {
            if let Some(a_s_) = &a_task.time {
                if s_.len() as u16 >= a_s_.len() as u16 {
                    s_.len() as u16
                } else {
                    a_s_.len() as u16
                }
            } else {
                s_.len() as u16
            }
        } else if let Some(a_s_) = &a_task.time {
            a_s_.len() as u16
        } else {
            0u16
        };

        meta.status = if let Some(s_) = &task.status {
            if let Some(a_s_) = &a_task.status {
                if s_.len() as u16 >= a_s_.len() as u16 {
                    s_.len() as u16
                } else {
                    a_s_.len() as u16
                }
            } else {
                s_.len() as u16
            }
        } else if let Some(a_s_) = &a_task.status {
            a_s_.len() as u16
        } else {
            0u16
        };

        meta
    }

    fn meta_vs_meta_to_meta(meta: &mut Meta, a_meta: &Meta) {
        meta.id = if meta.id >= a_meta.id {
            meta.id
        } else {
            a_meta.id
        };
        meta.name = if meta.name >= a_meta.name {
            meta.name
        } else {
            a_meta.name
        };
        meta.status = if meta.status >= a_meta.status {
            meta.status
        } else {
            a_meta.status
        };
        meta.date = if meta.date >= a_meta.date {
            meta.date
        } else {
            a_meta.date
        };
        meta.time = if meta.time >= a_meta.time {
            meta.time
        } else {
            a_meta.time
        };
    }

    fn task_to_meta(task: &Task, meta: &mut Meta) {
        let lid: u16 = task.id.checked_ilog10().unwrap() as u16 + 1;
        meta.id = if lid >= meta.id { lid } else { meta.id };
        meta.name = if task.name.len() as u16 >= meta.name {
            task.name.len() as u16
        } else {
            meta.name
        };

        meta.date = if let Some(s_) = &task.date {
            if s_.len() as u16 >= meta.date {
                s_.len() as u16
            } else {
                meta.date
            }
        } else {
            meta.date
        };
        meta.time = if let Some(s_) = &task.time {
            if s_.len() as u16 >= meta.time {
                s_.len() as u16
            } else {
                meta.time
            }
        } else {
            meta.time
        };
        meta.status = if let Some(s_) = &task.status {
            if s_.len() as u16 >= meta.status {
                s_.len() as u16
            } else {
                meta.status
            }
        } else {
            meta.status
        };
    }

    fn string_to_task(line: &str) -> Task {
        let mut task = Task::default();

        line.split(WORD_SEPERATOR).for_each(|x| {
            if x.starts_with("@") {
                task.id = x.strip_prefix("@").unwrap().parse().unwrap();
            } else if x.starts_with("d:") {
                task.date = Some(x.strip_prefix("d:").unwrap().to_string());
            } else if x.starts_with("t:") {
                task.time = Some(x.strip_prefix("t:").unwrap().to_string());
            } else if x.starts_with("s:") {
                task.status = Some(x.strip_prefix("s:").unwrap().to_string());
            } else if x.starts_with("n:") {
                task.name = x.strip_prefix("n:").unwrap().to_string();
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

    fn meta_to_string(meta: &Meta) -> String {
        let line = format!(
            "i:{},n:{},d:{},t:{},s:{}",
            meta.id, meta.name, meta.date, meta.time, meta.status
        );

        line
    }

    fn string_to_meta(line: String) -> Meta {
        let mut meta = Meta::new();

        line.split(",").for_each(|x| {
            if x.starts_with("i:") {
                meta.id = x.strip_prefix("i:").unwrap().parse().unwrap();
            } else if x.starts_with("n:") {
                meta.name = x.strip_prefix("n:").unwrap().parse().unwrap();
            } else if x.starts_with("d:") {
                meta.date = x.strip_prefix("d:").unwrap().parse().unwrap();
            } else if x.starts_with("t:") {
                meta.time = x.strip_prefix("t:").unwrap().parse().unwrap();
            } else if x.starts_with("s:") {
                meta.status = x.strip_prefix("s:").unwrap().parse().unwrap();
            }
        });

        meta
    }
}
