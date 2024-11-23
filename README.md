# metal-deploy

A lightweight tool for managing and deploying Git-based applications on bare metal systems.

## Features

- `$ metal-deploy add <git_repo_url>`: Adds a service, creates a user based on the repo name, and clones it into the user folder.
- `$ metal-deploy build <git-repo/service_name>`: Executes the repository's `./build.sh` file, and provides an interactive `.env` creator.
- `$ metal-deploy start <git-repo/service_name>`: Runs `./run.sh` and configures the service for deployment, ensuring it starts on boot.
- `$ metal-deploy stop <git-repo/service_name>`: Stops the running service.
- `$ metal-deploy delete <git-repo/service_name>`: Removes all associated user files, folders, and dependencies.

## How it works

This tool is pretty simple, it's goal is just to abstract a few commands for repeatability sake:

1. Create a new user and user folder for the service.
2. Clone the repository into the user's folder using the latest release.
3. Execute `./build.sh` to create a `./build` folder containing `./build/run.sh`.
4. Create a `.env` file for environment configuration based on a repo/.metal-deploy.env file.
5. Generate a user service file using paths to `./.env` and `./build/run.sh`.
6. Mount the service to the system and configure it to run on boot with systemd.

That's the core of things.

For each service, a new user will be created with a dedicated folder in `/home/`, like `/home/example` for the service 'example' or its repository URL. The service will only have access to its own directory, ensuring it's isolated and secure. When `metal-deploy build` is run, a `.env` file will be created in the service's home directory for use by the user service.

The service repo is expected to have at least:
- `build.sh`: Script that generates a `./build` folder containing a `run.sh` file to start the service.
- `.metal-deploy.env`: Template file with example environment variables; unpopulated variables will prompt the user for input.

metal-deploy assumes projects have a `build.sh` script that builds a `build` folder containing everything an application needs to run, including a `run.sh` file.

## TODO:

- 'update' command to pull the latest release of a Git repository to update a service, should be on latest release of a git repo.
- Automatically creates user and user-specific directories for each service.
- Implement a job scheduler that automatically pulls and updates the repository every 24 hours/time interval for automatic deployments.
- Integrate with CI/CD pipelines (GitHub Actions/GitLab CI/CD) through webhooks or SSH to automate release updates and deployments.
- Manage multiple deployments of different branches of a given service.


# Notes:
Some handy commands for debugging:

- `cut -d: -f1 /etc/passwd` - list all users
