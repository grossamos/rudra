+++
draft = false
weight = 100
description = "This guide covers how you can quickly get started using Rudra."
title = "Quick Start"
bref = "This guide covers how you can quickly get started using Rudra."
toc = false
+++

## Prerequisites

The following components are needed to get started using Rudra:
1. A REST-based web-Application
2. An OpenAPI Spec
3. API integration tests in your CI-Pipeline

## Step 1: Point your integration tests to rudra

First point your integration tests to the Rudra reverse proxy.
This ensures that Rudra can analyze an interpret requests happening during your integration tests.

Point your tests towards `http://localhost:13750` (without Docker) or `http://rudra:13750` (with Docker).
When using Docker Rudra will automatically configure networking for your container to connect to it.

## Step 2: Add the preperation step

Configure Rudra in a Preperation Stage.
This will set up Rudra for later use.
Place this preperation stage **after** you've started your service and **before** you'll run your integration tests.

Modify `openapi-source` to point to your openapi/swagger specification. This can also be a url.
Modify `instance-url` to point to the base of your service (everything before the basepath of your openapi spec).
Optionally set a desired `test-coverage` for your endpoints.

```yaml
  - name: init rudra
    uses: grossamos/rudra@v0.1.3
    with:
      stage: "preperation"
      openapi-source: "docs/swagger.json"
      instance-url: "http://localhost:8080"
      test-coverage: "75%"
```

## Step 3: Add the evaluation step

Place the evaluation stage after your integration tests have run.
The stage will evaluate any data observed during the testing phase and fail if the specified test coverage isn't met.

Rudra shouldn't be configured in this stage.

```yaml
  - uses: grossamos/rudra@v0.1.3
    name: eval rudra
    with:
      stage: "evaluation"
```

## Further information

Additional resources that could be usefull when using Rudra

* Example pipeline using Rudra: <https://github.com/grossamos/rudra-example>
* A complete overview of all Rudra configuration options:
* Rudra source code: <https://github.com/grossamos/rudra>




