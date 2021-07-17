# Blobe

Blobe is a simple cli web server manager application.

Web Server 
#42SkillsChallenge

## Folders Structure

- logs
    - instances
    - server
    - others-logs
- temp
    - history.txt
    - others-things
- intances
    - default
    - other-intance

    
# Modules

|   Module |            Description                      | Status |
|----------|---------------------------------------------|--------|
| instance | Module instance handles instance management | Dev    |

# Instance

|   Command |            Description                      | Status |
|----------|---------------------------------------------|--------|
| instance load {folder_name} |  If you need load instance after server already on, you can add instance to instances folder and load using instance folder name | implemented but in tests  |
| instance list | Show list of avaible instances | implemented but in tests  | 
| | | | 
 

# Ideias
Command pattern: >> {module} {cmd} {arg} {arg1}...

### Instance Commands

instance load {name} {folder_name || git_hub_link};

instance list -> List of registered instances

instance status -> See status of instance

instance new {name} {bind_addr} {port} {type} {addr} -> Create new intance

instance reload {name} -> Reload instance configs

instance stop {name} -> Stop instance http server

instance start {name} -> Start instance http server
