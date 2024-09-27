# Constant Integration hook for vanilla git

This project was created to run as a discord bot, notify you when a push has happened with the post-receive hook script.

## Install

### Move the post-receive script into your *bare* repositories hook directory
- this only works if the repository is pushed to

### create an event script
- this will be the script that runs after the hook fires
- any stdout from this file will be attached in a text file and sent to a channel that is set within the bot using the `>set_channel` command

### add any dependencies to the image in the Dockerfile
- ex : `RUN apk install --no-cache git`

### edit .env_template
- set DISCORD_TOKEN to your discord bots token
- set SERVER_PORT to your internal port
- rename this file to .env

### build the docker image
`docker buildx build -t ci-hooks:latest -f Dockerfile .`


## Run

To run the container requires 2 volumes
1. A data folder mounted to the volume /data which should be empty with RW access
2. A git credentials file mounted to /.git-credentials

### Docker compose
Edit the volumes in docker-compose.yml with the paths of the above files as well as your host port to whatever port you will allow through
`docker compose up -d`

### Docker run
Run with
`docker run -p <host_port>:6005/udp -v <host_data_folder>:/data -v <host_git_credentials>:/.git-credentials --name ci-hooks -d ci-hooks` 