# WizRD Backend

<img alt="wizrd.png" height="128" src="wizrd.png" width="128"/>

The WizRD Backend is an application comprised of three different modules, which expose the following services:

| Name            | Port | Description                                                                                            |
|-----------------|------|--------------------------------------------------------------------------------------------------------|
| `wizrd`         | 8000 | The Rocket Application that allows the frontend to retrieve data from the DB                           |
| `wizrd-db`      |      | The database where the map and the associated graph are stored. Uses PostgreSQL, PostGIS and PgRouting |
| `wizrd-pgadmin` | 8081 | A PgAdmin instance for debugging purposes                                                              |

The containers talk to each other through an external network named `wizrd-net`.

## Cloning

This repository uses nested submodules. If you didn't clone with `--recurse-submodules`, you can use the following command to fetch all submodules:

```console
$ git submodule update --init --recursive
```

## Bringup

A `docker-compose.yml` file that can be used to automate the deployment is provided for convenience. Remember to create the network before bringing up the containers.

```console
# cd wizrd-preprocessing
# ./get_map.sh
# cd ..
# docker network create wizrd-net
# docker compose up
```

A Dockerfile to automate the build and deployment of the application is provided for convenience. Super-user privileges are needed in order to use the Network.

> If you're using `podman`, just `/s/docker/podman/` or `alias docker=podman`.

## Debugging

If you want to run the application on your local machine (e.g., via IntelliJ) for debugging puposes, you should change the URL in `Rocket.toml` to `localhost` instead of `wizrd-db`, expose port `5432:5432` on the DB's container, and comment out the entry for container `wizrd` in `docker-compose.yml`.