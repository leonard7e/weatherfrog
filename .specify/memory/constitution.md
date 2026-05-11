# WeatherFrog Constitution

<!-- Constitution for WeatherFrog CLI application using Zero Cost Weather API -->

## Core Principles

### I. Zero Cost API Dependency (NON-NEGOTIABLE)

The application MUST use only the Zero Cost weather API as the sole external data source.

- No paid API services or fallback providers are permitted
- All weather data must originate from Zero Cost API exclusively
- API integration must handle rate limits, errors, and unavailable data gracefully
- If Zero Cost API becomes unavailable, the application MUST fail gracefully with clear error messages
- No caching mechanisms that could serve stale or incorrect data without explicit user consent

**Rationale**: The project is explicitly scoped to Zero Cost API; introducing paid alternatives violates the core purpose.

### II. CLI-First Interface

Every feature MUST be accessible via a command-line interface with text-based input/output.

- All user interaction occurs through stdin/stdout and command-line arguments
- Output formats MUST include both human-readable text and machine-parseable JSON
- Commands MUST be intuitive and follow consistent naming conventions
- Help documentation MUST be available via `--help` or `-h` flags on all commands
- Exit codes MUST follow POSIX conventions (0 for success, non-zero for errors)

**Rationale**: CLI-first design ensures scriptability, automation, and predictable behavior across platforms.

### III. Test-First Development (NON-NEGOTIABLE)

Tests MUST be written and approved before any implementation code.

- TDD (Test-Driven Development) is mandatory for all features
- Red-Green-Refactor cycle MUST be strictly enforced
- Unit tests MUST achieve high coverage for all business logic
- Integration tests MUST verify API interactions and CLI output
- No code merges without passing test suite

**Rationale**: Test-first ensures reliability and prevents regressions in a data-critical application.

### IV. Code Quality and Maintainability

All source code MUST be well-documented, readable, and maintainable by human developers.

- Every module, function, and class MUST have meaningful comments explaining purpose and behavior
- Public APIs MUST include documentation strings or comments
- Complex logic MUST be accompanied by explanatory comments
- Code MUST follow consistent style guidelines (enforced via linters/formatters)
- Variable and function names MUST be descriptive and self-documenting
- No dead code, unused imports, or commented-out code blocks

**Rationale**: Clear, documented code reduces maintenance overhead and enables future contributors to understand and extend the system.

### V. Error Handling and User Experience

The application MUST provide clear, actionable error messages and handle all failure scenarios gracefully.

- All errors MUST be logged with sufficient context for debugging
- User-facing errors MUST be clear and suggest remediation steps
- Network failures, API errors, and invalid inputs MUST be handled explicitly
- The application MUST never crash unexpectedly or leave the user in an unknown state
- All error conditions MUST have appropriate exit codes

**Rationale**: Robust error handling ensures reliability and provides a professional user experience even when things go wrong.

## API Integration Standards

### Zero Cost API Contract

- All API requests MUST include proper authentication (if required by Zero Cost API)
- Requests MUST respect rate limits and implement exponential backoff on failures
- Response parsing MUST validate data structure before use
- Invalid or missing data MUST trigger graceful fallback or clear error messages
- API version changes MUST be tracked and compatibility maintained

### Data Handling

- Weather data entities MUST be clearly defined and validated
- All external data MUST be treated as untrusted until validated
- Data transformations MUST be explicit and documented
- Timezone handling MUST be consistent and clearly documented

## Development Workflow

### Branch Strategy

- All work MUST occur on feature branches named `feature/[ticket-id]-[description]`
- Feature branches MUST be rebased on main before merging
- Pull requests MUST include test coverage and documentation updates

### Code Review

- All changes MUST be reviewed by at least one other developer
- Reviews MUST verify compliance with all constitution principles
- Code quality checks (linting, formatting, tests) MUST pass before review

### Commit Guidelines

- Commits MUST be atomic and focused on a single change
- Commit messages MUST follow conventional commits format
- Commits MUST reference relevant tickets or issues

## Governance

This constitution supersedes all other development guidelines for the WeatherFrog project.

**Amendment Process**:
- Any changes to this constitution require a formal amendment proposal
- Amendments MUST document the reason, impact, and migration plan
- All core principles require unanimous approval from project maintainers
- Non-principle sections require majority approval

**Compliance Review**:
- All PRs MUST include a constitution compliance checklist
- Deviations MUST be justified with complexity tracking in the implementation plan
- Regular audits SHOULD be conducted to ensure ongoing compliance

**Versioning Policy**:
- MAJOR: Backward incompatible principle removals or redefinitions
- MINOR: New principles or sections added, material guidance expansions
- PATCH: Clarifications, wording refinements, typo fixes

**Version**: 1.0.0 | **Ratified**: 2026-05-11 | **Last Amended**: 2026-05-11
