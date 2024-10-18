use tasker::task::Tasker;

fn main() -> Result<(), std::io::Error> {
    let mut tasker = Tasker::new();
    tasker.setup();
    tasker.run()?;

    Ok(())
}
