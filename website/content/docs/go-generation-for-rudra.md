+++
draft = false
weight = 300
description = "This guide explains how to generate gin/golang OpenAPI specs for rudra."
title = "Go OpenAPI generation"
bref = "This guide explains how to generate gin/golang OpenAPI specs for rudra."
toc = false
+++

## Prerequisites

You will need the following components to get started with using rudra:
1. A golang REST API written with the [Gin](https://github.com/gin-gonic/gin) framework
2. Integration tests for the OpenAPI spec

## Step 1: Download Swag

Download the swag binary from [Github](https://github.com/swaggo/swag/releases).

Then add the binary to your path. 
You should now be able to check if it's installed by running `./swag --version`.

## Step 2: Add dependencies

In your project embedd the nessicary packages:

```golang
import "github.com/swaggo/gin-swagger"
import "github.com/swaggo/files"
```

## Step 3: Add general API annotation

Add basic annotations to your main function.
These annotations will provide the basic information needed for the api.

For a sample API (taken from `rudra-example`) these annotations could look as follows:
```golang
// @title           Rudra Example Project
// @version         1.0
// @description     This is a sample project for the rudra test tool

// @license.name  BSD-2-Clause
// @license.url   https://raw.githubusercontent.com/grossamos/rudra/main/LICENSE

// @host      localhost:8080
// @securityDefinitions.basic  BasicAuth
// @BasePath  /
func main() {
    ...
}
```

For more information on all the annotations you can provide, see the [Swag documentation](https://github.com/swaggo/swag#how-to-use-it-with-gin).

## Step 4: Add API annotations

Now add specific information on all base URLs.

- `@Summary` provides short information on the endpoint
- `@Description` provides a full description of the endpoint
- `@Produce` indicates what format the responses are in
- `@Security` indicates what securityDefinition should be used for the endpoint (requires a securityDefinition in general annotation)
- `@Success` can be created multiple times for each "success" case of the endpoint
- `@Failure` can be created multiple times for each "failure" case of the endpoint
- `@Router` indicates what path the endpoint is on

In [rudra-example](https://github.com/grossamos/rudra-example) an example for an annotation is:
```golang
// Validate Weather State godoc
// @Summary      validates weather state
// @Description  checks if a given state is a valid weather state
// @Produce      json
// @Success      200  {object}  controller.IsValid
// @Failure      400  {object}  util.ErrorMessage
// @Router       /validate [post]
func ValidateWeather(c *gin.Context) {
    ...
}
```

## Step 5: Generate OpenAPI spec

Generate the OpenAPI spec with swag.

```bash
swag init
```

The generated spec can now be found under `./docs`.
In order to use it with rudra, add it to your git repository.

## Step 6: Initialize rudra

With your OpenAPI spec generated you can move on to adding rudra to your CI-Pipeline.
For this you can follow our [quickstart guide](/docs/quick-start/).

## Other Resources

- [Quickstart guide](/docs/quick-start/) for creating a basic rudra pipeline
- [Swag](https://github.com/swaggo/swag), the tool used to generate openapi docs for golang


