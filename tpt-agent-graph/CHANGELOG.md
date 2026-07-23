# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release: `AgentGraph`/`Node`/`NodeOutcome` state-machine executor
  with cycle support guarded by a step budget, and `ToolRegistry` for
  dispatching a model's tool call to a handler by name.
