# PromQL Tenant Proxy 🦀

A high-performance, lightweight HTTP proxy that extracts labels from PromQL labels and converts them into upstream HTTP headers. Built with Rust for sub-millisecond latency and memory safety.

---

## Overview

1. Intercepting the `/api/v1/query` and `/api/v1/query_range` requests.
2. Parsing the PromQL into an Abstract Syntax Tree (AST).
3. Extracting a specific label (e.g., `env`).
4. Injecting that value as a header before forwarding the request to the upstream storage.


## Features

* True AST Parsing: Uses `promql-parser` to navigate complex queries, including nested functions, aggregations, and binary expressions.
