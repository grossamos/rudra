+++
draft= false
title = "FAQ"
description = "Asked and answered"
+++

## Why does Rudra require OpenAPI?

Rudra requires requires an OpenAPI spec for endpoint discovery.
Without one, it could not reliably determine which endpoints exist in your application.
Since this information is nessicary for generting accurate test coverage, rudra requires an OpenAPI spec.

## Is rudra compatible with my language/web-framework?

Rudra is compatible with close to every language and framework that can be used to develop RESTfull applications.
If REST endpoints can be created with your language/framework, it is compatible with Rudra.

The only requirement is that you provide an OpenAPI spec.
Conversly non-RESTfull applications, using technologies such as GraphQL, aren't compatible with Rudra.

## Is rudra compatible with my testing framework?

Rudra is compatible with any API-testing framework that you can point towards its reverse proxy.
Examples for compatible testing frameworks include postman/newman, insomna and JMeter.

## Why enforce coverage for integration tests?

When creating tests for software, developers aim to acheive the highest test coverage possible.
Covering every use case is vital to finding bugs and problems before they enter production.

For reasons of quality controll, many organisations enforce minimum test coverage.
When creating unit tests, there are a myriad of tools for determining this test coverage.
However no generic tools for integration tests exist.
That's where Rudra comes in to fullfill this need.
