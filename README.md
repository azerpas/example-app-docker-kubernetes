# Voting App with Docker Compose and Kubernetes

Heavily inspired [by](https://github.com/dockersamples/example-voting-app)

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

The `vote` web app is then available on port 31000 on each host of the cluster, the `result` web app is available on port 31001. They're both accessible at [http://localhost:31000](http://localhost:31000) and [http://localhost:31001](http://localhost:31001).

To remove them, run:

```shell
kubectl delete -f k8s-specifications/
```

## Architecture

![Software4Cloud drawio](https://user-images.githubusercontent.com/19282069/227053374-04c64f4c-21c5-469d-8488-7daa7e74c14c.png)

The app is composed of 5 services:
- `vote` - a [`Next.js`](https://nextjs.org/) web app which lets you vote between two options; Dogs or Cats. This app is written in *TypeScript*, and uses *React* for the UI. It sends the vote to the `queue` service.
- `queue` - a [`Redis`](https://redis.io/) queue which collects new votes. Redis is a key-value store, it can be used as a database, cache and message broker, but is used here as a simple message queue, used to store the votes that are sent by the `vote` app. 
- `worker` - a [`Rust`](https://www.rust-lang.org/) worker which consumes votes and stores them in a persistent database. The worker is a simple application that reads a vote from the `queue` and stores it in the `db` service.
- `db` - a [`PostgreSQL`](https://www.postgresql.org/) database. The database is used to store the votes. It is backed by a Docker volume, so its data is persisted when the service is restarted.
- `result` - a [`Node.js`](https://nodejs.org/en/) web app which shows the results of the voting in real time. The `result` app is a simple web application that reads the votes from the `db` service and displays them.
