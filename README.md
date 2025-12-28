# PromQL Tenant Proxy 🦀

A high-performance, lightweight HTTP proxy that extracts tenant information from PromQL labels and converts them into upstream HTTP headers. Built with **Rust** for sub-millisecond latency and memory safety.

---

## 🚀 Overview

Many multi-tenant time-series databases (like Grafana Mimir, Cortex, or VictoriaMetrics) require a tenant identifier in the HTTP headers (e.g., `X-Scope-OrgID`). However, many clients only support filtering via PromQL labels.

**PromQL Tenant Proxy** solves this by:
1. Intercepting the `/api/v1/query` and `/api/v1/query_range` requests.
2. Parsing the PromQL into an **Abstract Syntax Tree (AST)**.
3. Extracting a specific label (e.g., `environment` or `tenant`).
4. Injecting that value as a header before forwarding the request to the upstream storage.



## ✨ Features

* **True AST Parsing:** Uses `promql-parser` to navigate complex queries, including nested functions, aggregations, and binary expressions.
* **Blazing Fast:** Leverages `Axum` and `Tokio` for non-blocking I/O.
* **Minimal Footprint:** No garbage collection and efficient memory management, making it ideal for sidecar deployments.
* **Safe by Design:** Rust's borrow checker prevents common memory-related vulnerabilities and race conditions.
