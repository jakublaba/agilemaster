# Agilemaster
Agilemaster is a command-line tool for generating dummy data for Kanban projects on Jira.
The use-case for this project is quickly bootstrapping examples for teaching purposes.

# Prerequisites
Before you can start using agilemaster, you need to create a `.json` file describing your user data.
No specific naming or path is required for this file.
```json
{
  "name": "jakublaba",
  "groups": [],
  "active": true,
  "email": "jakub.maciej.laba@gmail.com",
  "fullname": "Jakub Łaba"
}
```
**Warning:** Please note that `fullname` is spelled all lowercase, not camelCase

Unfortunately, agilemaster does not currently support automatic creation of projects, so creating an empty project manually is required.
On you Jira instance, go to `Projects -> Create project -> Software development -> Kanban`, make sure to remember the name given to the project.
Whether Team-Managed or Company-Managed is selected doesn't matter.

# Usage
Once user data `.json` and empty project on your Jira instance are created, you can use agilemaster.
```
agilemaster \
--name [name of your project - must match the name of project from your Jira instance] \
--author [fully qualified name of the .json file with user data] \
--start [project start date in dd-mm-YY format] \
--end [project end date in dd-mm-YY format] \
--issue-amount [total amount of issues/tasks to generate in the project]
```
To display this help, you can use `agilemaster --help`.
Note that each flag also supports a shorthand for convenience: `--name -> -n`, `--issue-amount -> -i`, `--help -> -h`, etc.
