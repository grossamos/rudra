# Rudra
Rudra is a multiligual test coverage analysis tool.
It allows teams to set and enforce coverage levels for integration tests in CI/CD-pipelines.

**NOTE: Rudra is still under heavy development and not yet stable or even feature complete**

## Local development
```bash
docker run -it -p 8080:80 --network rudra --name rudra --rm Rudra
docker run -it --name app --rm --network rudra -d rudra-example

docker exec -it rudra nginx -s stop
```

## Roadmap

### Future usecases
- Two ways to run it:
    1. Two parter with bash script (script to start docker container in background then action to do analysis)
    2. With start server command as input (DEFAULT)

- online openapi
- networking: automatically add network to docker commands (potentially even docker-compose)

### Documentation
- Document how networking can work (docker network with name or localhost:xyz)
