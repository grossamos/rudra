# Rudra
Rudra is a multiligual test coverage analysis tool.
It allows teams to set and enforce coverage levels for integration tests in CI/CD-pipelines.

**NOTE: Rudra is still under heavy development and not yet stable or even feature complete**

## Local development
```bash
docker run --env RUDRA_APP_BASE_URL=http://172.17.0.1:8080 --env RUDRA_OPENAPI_PATH=/swagger.yaml --volume $PWD/test/resource/swagger.yaml:/swagger.yaml -p 13750:80 --network rudra --name rudra --rm --env RUDRA_DEBUG=0 --env RUDRA_ACCOUNT_FOR_SECURITY=1 rudra
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
