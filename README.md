# Agilemaster
Agilemaster is a command-line tool for generating dummy data for Kanban projects on Jira.
The use-case for this project is quickly bootstrapping examples for teaching purposes.

# Prerequisites
Before you can start using agilemaster, you need to create a `.json` file describing your user data.
No specific naming or path is required for this file.
```json
{
  "name": "sampleusr",
  "groups": [],
  "active": true,
  "email": "user@example.com",
  "fullname": "John Smith"
}
```
You also need to create a `.json` file with authentication data for your Jira instance, since its rest api is used to extract some instance-specific data.
```json
{
  "email": "user@example.com",
  "apiKey": "secret-key",
  "url": "https://my-instance.atlassian.net"
}
```
**Warning:** Please note that `fullname` is spelled all lowercase, not camelCase

Unfortunately, agilemaster does not currently support automatic creation of projects, so creating an empty project manually is required.
On you Jira instance, go to `Projects -> Create project -> Software development -> Kanban`, make sure to remember the name given to the project.
Whether Team-Managed or Company-Managed is selected doesn't matter.

# Usage
Once user data `.json` and empty project on your Jira instance are created, you can use agilemaster.
```
Options:
  -n, --name <NAME>             Name of the generated project
  -a, --author <PATH>           Fully qualified name (with path) of json file with user data
  -a, --auth <PATH>             Fully qualified name (with path) of json file with authentication data
  -s, --start <DATE>            Start date of the project (dd-mm-YYYY)
  -e, --end <DATE>              End date of the project (dd-mm-YYYY)
  -i, --issues <AMOUNT>         Amount of issues to generate
  -s, --statuses <STATUSES>...  Space-separated list of statuses available in project
  -h, --help                    Print help
  -V, --version                 Print version
```
To display this help, you can use `agilemaster --help`.
