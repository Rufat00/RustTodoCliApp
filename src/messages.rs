pub const HELP: &str = "\nTodo Cli app by Rufat(https://github.com/Rufat00)\n\n\
    reset -f: Resets data file. Use -f to delete immediately. Note that this will delete all todos permanently.\n\
    list: Display the list of tasks.\n\
    new {text} -d/-n: Create a new task with the specified text. Use -d for 'done' or -n for 'not done' (default).\n\
    remove {id}: Remove a task with the specified ID.\n\
    update {id} {text} -d/-n: Update the text or status (done/not done) of a task with the specified ID. Use -d for 'done' or -n for 'not done'.\n\
    done {id}: Mark a task as 'done' with the specified ID.\n\
    notdone {id}: Mark a task as 'not done' with the specified ID.
";
