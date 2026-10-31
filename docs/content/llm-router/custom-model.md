---
title: "Custom model routing"
available: false
---

# Custom model routing

A custom model has a caller-facing name and an ordered list of configured provider/model pairs. Routing, persistence, health detection, and serving telemetry remain planned.

## Route selection

Route order defines priority. Exclude down or inactive providers from serving. Each route identifies both a configured connection and an upstream model; a provider name alone is insufficient. Credentials stay within the connection configuration.

Observed serving percentages are read-only telemetry, not traffic weights. A first-priority provider can have a zero observed share if unavailable throughout the reporting period. An exhausted provider may retain a nonzero historical share. Measurement windows, aggregation, exhaustion detection, and recovery thresholds remain unresolved.

Validate model capabilities and endpoint compatibility across all routes. Protocol translation cannot supply capabilities absent from an upstream model. A model requires a name and at least one complete route; route pairs are distinct. System-prompt merging, name uniqueness, permissions, and lifecycle rules remain unresolved.

## Optional pricing

Custom input, output, and cache-read token prices use USD per million tokens. Unset pricing selects automatic resolution; explicit zero is a configured rate. Price precedence, partial overrides, automatic resolution, and charging behavior remain unresolved.

## Failure boundaries

Define eligible fallback errors, per-attempt and total timeouts, retry limits, backoff, exhaustion responses, cancellation, and partial-stream handling before implementation. Do not silently restart a partially delivered response. Usage attribution across attempts belongs to the [Gateway](../llm-gateway/adr.md).

See the [Router architecture](adr.md).
