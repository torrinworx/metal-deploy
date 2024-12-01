# metal-deploy

A simple system for managing, building, and deploying git repos on bare metal.

Using the linux user system, git, systemctl, and systemd, we can acheive almost all the benifets of docker and other containerization tools out there, with the advantage of running applications directly on the metal of a server without the need for a virtualization layer.

## Features
- **Add a Service**: `$ metal-deploy add <git_repo_url>`
	- Adds a new service, creating a user based on the repository name and cloning the repository into the services dedicated user in /home folder.
- **Build a Service**: `$ metal-deploy build <git-repo/service_name>`
	- Executes the `build.sh` file from the repository to prepare the service, and provides an optional interactive prompt to create a `.env` file for configuring environment variables.
- **Start a Service**: `$ metal-deploy start <git-repo/service_name>`
	- Runs the `run.sh` script to start the service, and setting it up to start on boot.
- **Stop a Service**: `$ metal-deploy stop <git-repo/service_name>`
	- Stops a running service.
- **Delete a Service**: `$ metal-deploy delete <git-repo/service_name>`
	- Removes a service, including all associated user files, folders, and dependencies from the system.
- **Restart a Service**: `$ metal-deploy restart <git-repo/service_name>`
	- Restarts a service.
- **Update a Service**: `$ metal-deploy update <git-repo/service_name> [--latest-release]`
	- Updates a service with the latest changes from the repository. You can specify `--latest-release` to update to the latest release version. By default it will simply `git pull` the latest changes of the repo.
- **List Services**: `$ metal-deploy list`
	- Displays a list of all services on the system.

## How it Works

`metal-deploy` is designed to simplify the deployment process by managing each service as a standalone user. Here's how it operates:

1. **Repository Cloning**: The specified git repository is cloned into the service's home directory (created on clone from the git url) using the latest release or chosen branch. This provides the base code for the service.

2. **User Creation**: For each service, a new user account is created along with a dedicated directory in `/home/` (e.g., `/home/example`). Isolation and security is inherited by giving the service access only to its directory.

3. **Build Process**: Within the cloned repository, the `build.sh` script is executed. This script should compile or set up the application and produce a `./build` folder containing everything it needs to run; which includes a `run.sh` script to start the service.

4. **Environment Setup**: A `.env` file is generated based on the `.metal-deploy.env` template found in the repository. This file allows customization of environment variables, prompting user input for any unspecified values. If no .metal-deploy.env file is found, deployment will continue.

5. **Service Configuration**: A systemd service file is created for the user, pointing to the `./.env` file and `./build/run.sh`. This makes the service ready to run and ensures that it starts automatically on system boot.

Note: `metal-deploy` requires the repository to include:
- A `build.sh` script that generates a `build` directory with all necessary files, including a `run.sh` script to launch the service.
- A `.metal-deploy.env` template to define environment variables needed for the service. Unspecified variables in this file will prompt user input during setup.

## TODO:
- Service branch cloning implementation: Add a given services main repo, specify the branch to launch within the `build` and `start` commands themselves.
- Implement a job scheduler that automatically pulls and updates the repository every 24 hours/time interval for automatic deployments.
- Integrate with CI/CD pipelines (GitHub Actions/GitLab CI/CD) through webhooks or SSH to automate release updates and deployments.
- Manage multiple deployments of different branches of a given service.

- Policy on distros and versions. Right now this has been tested and built on Ubuntu 24.04. How do we handle running applications that work on other distros/base images? Should we just assume everything to run on Ubuntu?
- wrap existing systemctl service commands:
    - `restart {service_name}`
    - `enable {service_name}`
    - `disable {service_name}`
    - `mask {service_name}`
    - `status (combo of /is-active and is-enabled imo these can be a single command) {service_name}`
    - `kill {kill_type: "hard" = 9, "soft" = 15} {service_name}`

- list deploy, deploy a list of repos automatically similar to docker compose.

# Notes:
Some handy commands for debugging:

- `cut -d: -f1 /etc/passwd` - list all users

Setup a working demo with OpenGig:
`sudo cargo run -- add https://github.com/torrinworx/OpenGig.org.git -b metal-deploy`

Mongodb for local testing with my ssh chromebook:
DB=mongodb://192.168.1.66:27017

I'm running tests for this thing on an acer C720 chromebook, which can run OpenGig, metal-deploy, but it can't run mongodb lol, so I have that hosted on my desktop and need that url to connect, but it works better than bricking my desktop accidentally again.

export XDG_RUNTIME_DIR="/run/user/$(id -u)"
export DBUS_SESSION_BUS_ADDRESS="unix:path=${XDG_RUNTIME_DIR}/bus"

# Repo examples
These are working repos that can be ran with metal-deploy:
- [opengig.org](https://github.com/torrinworx/OpenGig.org/tree/metal-deploy)
- [metal-deploy-qbittorrent](https://github.com/torrinworx/metal-deploy-qbittorrent)
- [metal-deploy-prowlarr](https://github.com/torrinworx/metal-deploy-prowlarr)
- [metal-deploy-sonarr](https://github.com/torrinworx/metal-deploy-sonarr)
- [metal-deploy-lidarr](https://github.com/torrinworx/metal-deploy-lidarr)
- [metal-deploy-radarr](https://github.com/torrinworx/metal-deploy-radarr)
- [metal-deploy-readarr](https://github.com/torrinworx/metal-deploy-readarr)
