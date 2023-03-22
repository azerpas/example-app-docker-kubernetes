# Example Voting App

A vote application running in Docker Containers or in Kubernetes. 

## Getting started

The best way to deploy the application for development is to use `Docker Compose`:

```shell
docker compose up
```

The `vote` app will be running at [http://localhost:5151](http://localhost:5151), and the `results` will be at [http://localhost:5001](http://localhost:5001).

## Run the app in Kubernetes

Before running the the `kubectl` commands, you need to build the images:
```
docker build ./worker -t worker:1.0-alpha
docker build ./result -t result:1.0-alpha
docker build ./vote -t vote:1.0-alpha
```

The folder k8s-specifications contains the YAML specifications of the Voting App's services.

Run the following command to create the deployments and services.

```shell
kubectl create -f k8s-specifications/
```

The `vote` web app is then available on port 31000 on each host of the cluster, the `result` web app is available on port 31001.

To remove them, run:

```shell
kubectl delete -f k8s-specifications/
```

## Architecture

![Software4Cloud drawio](https://user-images.githubusercontent.com/19282069/227053374-04c64f4c-21c5-469d-8488-7daa7e74c14c.png)

* A front-end web app in [Next.js](/vote) which lets you vote between two options
* A [Redis](https://hub.docker.com/_/redis/) which collects new votes
* A [Rust](/worker/) worker which consumes votes and stores them in…
* A [Postgres](https://hub.docker.com/_/postgres/) database backed by a Docker volume
* A [Node.js](/result) web app which shows the results of the voting in real time
