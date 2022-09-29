+++
draft = false
weight = 200
description = "This page explains all possible configuration options for Rudra"
title = "Configuration Options"
bref = "This page explains all possible configuration options for Rudra"
toc = false
+++

Rudra offers many options to tailor it to your needs.
This page will explain all options in as much detail as possible.
If you'd like a faster introduction to Rudra you can check out our [quickstart guide](/docs/quick-start).

## Overview

The following is an overview of all options.
Each table entry corresponds to a property that can be set when creating an action.


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

## Detailed Information

### Multiple Services
Getting test coverage on a system with multiple services is also possible.
Instead of providing a single `instance-url`, `openapi-source` and `port`, you provide a mapping via the `services` option.

The mapping should be provided in the following format:
```yaml
instance-url; openapi-source; port;
instance-url; openapi-source; port;
// etc ...
```

Hereby ports have to be unique and can't be used twice.
The valid fields for `instance-url`, `openapi-source` and `port` are equal to their respective single options.

An example for a port mapping (taken from [rudra-example](https://github.com/grossamos/rudra-example)) looks as follows:
```yaml
    services: |
        http://localhost:8080; docs/swagger1.yaml; 13751;
        http://localhost:8443; docs/swagger2.yaml; 13752;
```


### Networking
Your integration tests can connect to rudra in two different ways:
1. `http://localhost:13750`
2. `http://rudra:13750`

The first configuration simply replaces the localhost text with the IP of the Docker container.

The second configuration creates a Docker network and adds all running docker containers to it.
When running integration tests from within a Docker container this option could be advatagious.

### Security Headers

Rudra can pick up on security annotations in an OpenAPI spec.
By default it ignores these.
With the options `account-for-security-forbidden` and `account-for-security-unautorized`, Rudra automatically requires you to check `401` and `403` errors respecively.

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
