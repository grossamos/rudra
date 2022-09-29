# Rudra
Rudra is an openapi based test coverage analysis tool.
It allows teams to set and enforce coverage levels for integration tests in CI/CD-pipelines.

Contributions to the project are allways welcome, please consult [CONTIBUTING.md](./CONTRIBUTING.md) for more information.

## Quickstart
#### Step 1: Point your integration tests to rudra
First point your integration tests to the Rudra reverse proxy.
This ensures that Rudra can analyze an interpret requests happening during your integration tests.

Point your tests towards `http://localhost:13750` (without Docker) or `http://rudra:13750` (with Docker).
When using Docker Rudra will automatically configure networking for your container to connect to it.

#### Step 2: Add a preperation step
Configure Rudra in a preperation Stage.
This will set up Rudra for later use.
Place this preperation stage **after** you've started your service and **before** you'll run your integration tests.

Remeber to replace the location of your OpenAPI spec and the instance URL.
The OpenAPI Spec can also be provided via a link.

```yaml
  - name: init rudra
    uses: grossamos/rudra@v0.1.3
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
  - uses: grossamos/rudra@v0.1.3
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
only-account-for-pr              | Indictates if only changes within a PR should be taken into account, doesn't take effekt outside a PR (default: `false`) | boolean | `true`
openapi-source                   | Location of openapi/swagger spec | Path or URL | `docs/swagger.yaml`
port                             | Port for rudra to listen on (default `13750`) | unsigned 16 bit integer | `13750`
services                         | Configuartion for multiple services, conflicts with port, openapi-source, instance-url | `instance-url; openapi-source; port;\n` | see [here](#multiple-services)
stage                            | Specifies which stage to use | `preperation`, `evaluation` | `preperation`
test-coverage                    | Coverage to enforce in evaluation stage (default `70%`) | Percentage or float | `0.75`, `75%`
groupings                        | Allows for certain configruations to be grouped together or ignored | `path; method; status_code; ignored;\n` | see [here](#groupings)

## Examples

## Rudra Example
A reference pipeline can be point under <https://github.com/grossamos/rudra-example>.
It uses a go service and postman to serve as an example of how to integrate rudra into your application.

The pipeline in rudra-example is structured as follows:
```yaml
  - uses: grossamos/rudra@v0.1.3
    name: init rudra
    with:
      stage: "preperation"
      openapi-source: "docs/swagger.json"
      instance-url: "http://localhost:8080"
      account-for-security-forbidden: true
      test-coverage: "90%"
      only-account-for-pr: true
# ... Integration tests ...
  - uses: grossamos/rudra@v0.1.3
    name: eval rudra
    with:
      stage: "evaluation"
```

### Multiple services
A configuration with multiple endpoints and openapi specifications could look as follows:
```yaml
services: |
    http://localhost:8080; docs/swagger1.yaml; 13751;
    http://localhost:8443; docs/swagger2.yaml; 13752;
```

### Groupings
Somtimes endpoints reuse the same logic and shouldn't need to be tested twice.
Other times some configurations simply can't get tested and need to be ignored from a perspective of test coverage.
The groupings feature allows you to define groups.
A group only requires a single test to make all endpoints count as tested.
If the `ignored` flag is set, they are assumed to be tested and are taken out of consideration.

An example could look as follows:
```yaml
groupings: |
    /foo/bar; GET; 200; true;
    /foo/{bar}/moo; GET, POST; 200, 418; false;
```


