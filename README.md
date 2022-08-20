# Rudra
Rudra is an openapi based test coverage analysis tool.
It allows teams to set and enforce coverage levels for integration tests in CI/CD-pipelines.

Contributions to the project are allways welcome, please consult [CONTIBUTING.md](./CONTRIBUTING.md) for more information.

**NOTE: rudra is still under development and not yet production ready**

## Quickstart
#### Step 1: Point your integration tests to rudra
For rudra to work, you have to point your integration tests to the rudra reverse proxy.
In postman for example this can be done by creating a new environment and modifying a base url.

Point your tests towards `http://localhost:13750` or `http://rudra:13750`.

#### Step 2: Add a configuration step
Place the rudra preperation stage **after** your service is running and **before** you'll run your integration tests.

```yaml
  - name: init rudra
    uses: grossamos/rudra@v0.1.1
    with:
      stage: "preperation"
      openapi-source: "docs/swagger.json"
      instance-url: "http://localhost:8080"
      test-coverage: "75%"
```

Modify `openapi-source` to point to your openapi/swagger specification. This can also be a url.

Modify `instance-url` to point to the base of your service (everything before the basepath of your openapi spec).

Optionally set a desired `test-coverage` for your endpoints.

#### Step 3: Add evaluation step
Place the rudra evaluation stage somewhere after your integration tests have run.

```yaml
  - uses: grossamos/rudra@v0.1.1
    name: eval rudra
    with:
      stage: "evaluation"
```
This stage will fail if test coverage isn't met and can display additional information gathered during the integration tests.

## Overview
Rudra works by acting as a reverse proxy between your application and integration tests.
It collects and compares the requests (and responses) with an openapi spec.

The reverse proxy is set up an configured in the first "preperation" stage.
Analysis and any propagation of results occurs during the "evaluation" stage.

### Configuration options
Option                           | Description | Values | Examples
---------------------------------|-------------|--------|---------
account-for-security-forbidden   | Take security annotations into account and require 403 cases to be handled (default `false`) | boolean | `true`
account-for-security-unautorized | Take security annotations into account and require 401 cases to be handled (default `false`) | boolean | `true`
debug                            | Enables Debug mode (default `false`) | boolean | `true`
instance-url                     | Base of service, excluding basepath from openapi | URL | `http://localhost:8080`
openapi-source                   | Location of openapi/swagger spec | Path or URL | `docs/swagger.yaml`
port                             | Port for rudra to listen on (default `13750`) | unsigned 16 bit integer | `13750`
services                         | Configuartion for multiple services, conflicts with port, openapi-source, instance-url | `instance-url; openapi-source; port;\n` | see [here](#multiple-services)
stage                            | Specifies which stage to use | `preperation`, `evaluation` | `preperation`
test-coverage                    | Coverage to enforce in evaluation stage (default `70%`) | Percentage or float | `0.75`, `75%`

## Examples
A reference pipeline can be point under <https://github.com/grossamos/rudra-example>.
It uses a go service and postman to serve as an example of how to integrate rudra into your application.

### Multiple services
A configuration with multiple endpoints and openapi specifications could look as follows:
```yaml
services: |
    http://localhost:8080; docs/swagger1.yaml; 13751;
    http://localhost:8443; docs/swagger2.yaml; 13752;
```

