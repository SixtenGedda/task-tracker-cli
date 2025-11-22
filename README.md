# Task Tracker CLI 
Simple command line task tracker written in rust.

## Features

- Add, update and delete tasks
- List tasks by status
- Tasks stored in JSON file

## How to run 
```bash
cargo run -- {ARGUMENT HERE}

```
### Update a task
```bash
task-tracker-cli update 1 "Buy groceries and cook dinner"
```

### Delete a task
```bash
task-tracker-cli remove 1
```

### Mark task status
```bash
task-tracker-cli mark-in-progress 1
task-tracker-cli mark-done 1
```

### List tasks
```bash
# List all tasks
task-tracker-cli list

# List by status
task-tracker-cli list done
task-tracker-cli list todo
task-tracker-cli list in-progress
```

