# metal-deploy

A lightweight system for managing and deploying Git-based applications as systemd services.

- Aut "update" by pulling the latest release of a git repo
- works like an open source app store/app/service manager for open source git repos
- configurable .env editor
- auto create users/user folder for each service
- assumes projets have a build.sh that builds a build folder containing all an application needs to run including a run.sh file

Abstract best practices for building and deploying git repos/projects on a system while simplifying the management of multiple systems.
1. Create new user for the service and user folder
2. git clone repo into user folder, latest release
3. $ ./build.sh -> ./build folder with ./build/run.sh
4. create .env file
5. create user service file with ./.env path and ./build/run.sh path
6 mount service to system and setup systemd for service to run on boot.

features:
- `$ metal-deploy add <git repo url>` - adds a service to list of services, creates a user for it given the repos name, clones the repo in the user folder
- `$ metal-deploy build <git-repo/service name>` - runs the git repos ./build.sh file, prompts the user with interactive .env creator, filled out variables are skipped, empty ones are filled out by user.
- `$ metal-deploy start <git-repo/service name>` - runs the services ./run.sh, starts and configures the service automatically so that it's deployed and will startup on boot.
- `$ metal-deploy stop <git-repo/service name>` - stops the service from running
- `$ metal-deploy delete <git-repo/service name>` - completely removes all users/service files/user folders and all dependencies needed to run the service.

The purpose of this tool is to make deployment of applications easy in production environments, both hobby and professional.

services must have:
- build.sh - build file that produces a ./build folder containing a run.sh file that starts the services
- .env.deploy.example - simple file with example env vars, vars left blank will automatically be asked by the user to be filled out on first start, settings pre-filled out are assumed to be functional in deployment. 

some features we should implement:
- a job schedular that can automatically pull and check the git repo for the latest release every 24 hours or so automatically. - Meant for apps you just deploy like jellyfin
- a way to allow github actions/gitlab ci/cd to either via webhooks or ssh, pull the latest release and build it, automatically removing the old setup and re-building the newest branch/release. this feature could also be expanded to build/deploy different versions of the same branch and become a sort of branch deployment tool.

New users for each service will be created, and their user folder will be simply in /home/, and each service folder like /home/example for service 'example' or https://github.com/example/example.git.

It is assumed that all files and everything that that service needs to run will be self contained in the /home/example folder, user permissions will be set to prevent all other system folders from being affected or manipulated in any way. The only folder service 'example' has access to is /home/example.

After a user runs `metal-deploy build` a simple .env file is created in the /home/example folder. This will be referenced by the user service.

Gonna try to build this in rust, we'll see...
