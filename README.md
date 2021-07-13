# Blobe
Web Server 
#42SkillsChallenge

## Folders

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

## Commands

instance list -> List of registered instances

instance status -> See status of instance

instance new {name} {bind_addr} {port} {type} {addr} -> Create new intance

instance reload {name} -> Reload instance configs

instance stop {name} -> Stop instance http server

instance start {name} -> Start instance http server

instance load {name} {folder_name}