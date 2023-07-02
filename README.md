# Simple Task manager in Rust using clap


For install you need just clone git repo:
``` git clone https://github.com/Act0r1/task_manager && cd task_manager```

After that you can should install it as follow:
``` cargo install --path . ```

After you can start using, remember, that it will be create a tasks.json in current dir, where you launch app.
For add new task:
``` task_manager add "Task name" ```

For edit task:
``` task_manager edit {number task to edit} "new name" ```

Number of tasks you can check via:
``` task_manager list ```
