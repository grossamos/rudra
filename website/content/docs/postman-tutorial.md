+++
draft = false
weight = 300
description = "Guide to setup rudra with postman"
title = "Rudra with postman"
bref = "Guide to setup rudra with postman"
toc = false
+++

## Prerequisites
At this point in creating your CI-Pipeline you should have the following:
- Running Application
- Accessible OpenAPI spec (via. URL or on disk)
- A postman collection to use

## Step 1: Create a new environment
First create a new environment for your postman test suite (clicking on new > environment).
Here change the port of your base URL.

![postman setup](/images/postman_environment_setup.png)

By default Rudra uses port 13750. 
This can be changed using the `port` parameter later (see. [configuration options](/docs/configuration-options.md) for more).

If you're not using environments, manually update the port on all your tests.

## Step 2: Configure Rudra
Configure Rudra in a Preperation Stage.
This will set up Rudra for later use.
Place this preperation stage **after** you've started your service and **before** you'll run your integration tests.

Modify `openapi-source` to point to your openapi/swagger specification. This can also be a url.
Modify `instance-url` to point to the base of your service (everything before the basepath of your openapi spec).
Optionally set a desired `test-coverage` for your endpoints.

```yaml
  - name: init rudra
    uses: grossamos/rudra@v0.1.2
    with:
      stage: "preperation"
      openapi-source: "docs/swagger.json"
      instance-url: "http://localhost:8080"
      test-coverage: "75%"
```

## Step 3: Run newman
Create a step for running newman after configuring rudra.

```yaml
  - uses: matt-ball/newman-action@master
    name: run integration tests
    with:
      collection: tests/rudra-example.postman_collection.json
      environment: tests/rudra-example-ci.postman_environment.json
```

Keep in mind to change your collection and environment as needed.
The environment parameter can also be omitted when not using one.

## Step 4:
Add the evaluation stage.
This stage will evaluate your tests after newman and fail if the configured test coverage wasn't met.
No configuration should be added to this stage.

```yaml
  - uses: grossamos/rudra@v0.1.2
    name: eval rudra
    with:
      stage: "evaluation"
```

## Result

Taken from [rudra-example](https://github.com/grossamos/rudra-example), a possible pipeline could look as follows:
```yaml
name: Integration Tests

on:
  push:
    branches: [ main ]
  workflow_dispatch:

jobs:
  run-integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: install nix
        uses: cachix/install-nix-action@v17
        with:
          nix_path: nixpkgs=channel:nixos-unstable
      # Starting the application
      - run: nix build .
      - run: nix run . &
      # Preparing Rudra
      - uses: grossamos/rudra@v0.1.2
        name: init rudra
        with:
          stage: "preperation"
          openapi-source: "docs/swagger.json"
          instance-url: "http://localhost:8080"
          test-coverage: "90%"
      # Running Newman
      - uses: matt-ball/newman-action@master
        name: run integration tests
        with:
          collection: tests/rudra-example.postman_collection.json
          environment: tests/rudra-example-ci.postman_environment.json
      # Evaluating test coverage
      - uses: grossamos/rudra@v0.1.2
        name: eval rudra
        with:
          stage: "evaluation"
```
