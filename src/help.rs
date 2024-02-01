const USAGE: &str = "Welcome to Riel!
Last help message update: 2024-1-28 by Yeray Romero
Usage: riel ([options]) [command] [arguments/subcommands]\n
Commands:
help: Shows this message.
mount: Mounts a Riel repository in the current directory.
commit: Commits changes to the repository.
add: Adds files to the repository.
clone: Clones a repository from a given URL.
goto: Goes to a commit, saving local files and not commiting anything yet.\n
sudo-destruct: For developer purposes, deletes the repository.\n
Remember Riel is still in development.";
pub(crate) fn generic_help() {
    println!("{}", USAGE);
}
