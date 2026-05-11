# Tasks: WeatherFrog CLI

**Input**: Design documents from `/specs/001-weather-cli/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests MUST be written first and fail before implementation (TDD mandatory per Constitution Principle III).

**Constitution Compliance**: All tasks MUST align with WeatherFrog Constitution v1.0.0:
- Zero Cost API only (no paid alternatives)
- CLI-first interface (text in/out, JSON support)
- Test-first development (TDD enforced)
- Code quality (meaningful comments, documentation)
- Error handling (graceful failures, clear messages)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Single-project Rust crate. All source under `src/`, tests under `tests/`.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency configuration

- [X] T001 Create Rust project structure with `cargo init --name weatherfrog`
- [X] T002 [P] Configure Cargo.toml with dependencies (clap, reqwest, serde, serde_json, tokio, dirs, thiserror, chrono)
- [X] T003 [P] Configure cargo fmt and clippy linting rules in Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 [P] Define WeatherError enum with all variants (Network, Api, Parse, Validation, Storage, Config) in `src/error.rs`
- [X] T005 [P] Define Location struct with validation and TemperatureUnit enum in `src/models/location.rs`
- [X] T006 [P] Define WeatherData struct with serde derives in `src/models/weather.rs`
- [X] T007 Create models module re-exports in `src/models/mod.rs`
- [X] T008 [P] Define API request/response types (geocoding, current weather, forecast) in `src/api/types.rs`
- [X] T009 Implement HTTP client wrapper with reqwest, async runtime, and base URL configuration in `src/api/client.rs`
- [X] T010 Create API module re-exports in `src/api/mod.rs`
- [X] T011 [P] Implement human-readable text output formatting for weather data in `src/output/text.rs`
- [X] T012 [P] Implement JSON output formatting with serde serialization in `src/output/json.rs`
- [X] T013 Create output module re-exports in `src/output/mod.rs`
- [X] T014 [P] Create config.rs with API base URL, default values, and exit code constants in `src/config.rs`
- [X] T015 Implement main.rs with clap derive CLI structure, subcommand enum, and dispatch logic

**Checkpoint**: Foundation ready — user story implementation can now begin

---

## Phase 3: User Story 1 - Fetch Current Weather (Priority: P1) 🎯 MVP

**Goal**: User can fetch current weather by city name or coordinates and see results in text or JSON format

**Independent Test**: User can run `weatherfrog fetch --city "London"` and see current temperature, conditions, humidity, and wind speed

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T016 [P] [US1] Unit tests for Location validation (valid names, invalid coordinates, empty strings) in `tests/unit/models_test.rs`
- [X] T017 [P] [US1] Unit tests for WeatherData serde serialization and deserialization in `tests/unit/models_test.rs`
- [X] T018 [P] [US1] Unit tests for text output formatting (correct alignment, unit labels) in `tests/unit/output_test.rs`
- [X] T019 [P] [US1] Unit tests for JSON output validity and field names in `tests/unit/output_test.rs`

### Implementation for User Story 1

- [X] T020 [US1] Implement fetch command handler (resolve location → call API → parse response → format output) in `src/commands/fetch.rs`
- [X] T021 [US1] Add --city, --latitude, --longitude, --json, and --unit clap flags to fetch subcommand in `src/main.rs`
- [X] T022 [US1] Wire fetch command into main.rs subcommand dispatch
- [X] T023 [US1] Add geocoding lookup (city name → coordinates) to API client in `src/api/client.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Fetch Forecast Data (Priority: P2)

**Goal**: User can view multi-day weather forecasts for a location

**Independent Test**: User can run `weatherfrog forecast --city "Berlin" --days 5` and see a 5-day weather forecast

### Tests for User Story 2 ⚠️

- [X] T024 [P] [US2] Unit tests for ForecastData and ForecastDay validation (max >= min, date range) in `tests/unit/models_test.rs`
- [X] T025 [P] [US2] Unit tests for forecast text output formatting (table alignment) in `tests/unit/output_test.rs`
- [X] T026 [P] [US2] Integration test for forecast command with mocked API response in `tests/integration/cli_test.rs`

### Implementation for User Story 2

- [X] T027 [P] [US2] Define ForecastData and ForecastDay structs with serde derives in `src/models/forecast.rs`
- [X] T028 [US2] Update `src/models/mod.rs` to export forecast types
- [X] T029 [US2] Add forecast API endpoint (multi-day weather data) to API client in `src/api/client.rs`
- [X] T030 [US2] Implement forecast command handler (resolve location → call forecast API → parse → format) in `src/commands/forecast.rs`
- [X] T031 [US2] Wire forecast command into main.rs subcommand dispatch
- [X] T032 [US2] Add --days, --city, --json, and --unit clap flags to forecast subcommand in `src/main.rs`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Handle API Errors Gracefully (Priority: P1)

**Goal**: All API and network errors are handled gracefully with clear user messages and no panics

**Independent Test**: When API is unavailable, user sees a clear error message with retry suggestions instead of a panic or crash

### Tests for User Story 3 ⚠️

- [X] T033 [P] [US3] Unit tests for WeatherError Display impl (user-friendly messages per variant) in `tests/unit/models_test.rs`
- [X] T034 [P] [US3] Integration test for network failure handling (connection refused, timeout) in `tests/integration/api_test.rs`
- [X] T035 [P] [US3] Integration test for API error response handling (non-200 status codes) in `tests/integration/api_test.rs`
- [X] T036 [P] [US3] Unit tests for exit code mapping from WeatherError variants in `tests/unit/models_test.rs`

### Implementation for User Story 3

- [X] T037 [US3] Implement exponential backoff retry logic (1s, 2s, 4s, max 3 retries) in `src/api/client.rs`
- [X] T038 [US3] Add user-friendly error message formatting for each WeatherError variant in `src/error.rs`
- [X] T039 [US3] Map WeatherError variants to POSIX exit codes (0-8) in main.rs error handling
- [X] T040 [US3] Add verbose/debug logging for error context when --verbose flag is set in `src/config.rs`

**Checkpoint**: Error handling is complete — all commands now fail gracefully with clear messages

---

## Phase 6: User Story 4 - Save and Retrieve Favorite Locations (Priority: P3)

**Goal**: User can save, list, remove, and use favorite locations for quick weather lookups

**Independent Test**: User can run `weatherfrog favorites add "Paris"` and then `weatherfrog fetch --favorite "Paris"` to quickly retrieve saved location

### Tests for User Story 4 ⚠️

- [X] T041 [P] [US4] Unit tests for UserPreferences serde serialization and deserialization in `tests/unit/models_test.rs`
- [X] T042 [P] [US4] Unit tests for favorites file I/O (read, write, duplicate detection) in `tests/unit/models_test.rs`
- [X] T043 [P] [US4] Integration tests for favorites add/list/remove commands in `tests/integration/cli_test.rs`

### Implementation for User Story 4

- [X] T044 [P] [US4] Define UserPreferences struct with serde derives in `src/models/preferences.rs`
- [X] T045 [US4] Update `src/models/mod.rs` to export preferences types
- [X] T046 [US4] Implement favorites file I/O (read/write JSON at `~/.weatherfrog/favorites.json`) in `src/storage/favorites.rs`
- [X] T047 [US4] Create storage module re-exports in `src/storage/mod.rs`
- [X] T048 [US4] Implement favorites add command (geocode → validate → save) in `src/commands/favorites.rs`
- [X] T049 [US4] Implement favorites list command (read → format → display) in `src/commands/favorites.rs`
- [X] T050 [US4] Implement favorites remove command (find → remove → save) in `src/commands/favorites.rs`
- [X] T051 [US4] Wire favorites subcommands (add, list, remove) into main.rs
- [X] T052 [US4] Add --favorite flag support to fetch and forecast commands in `src/main.rs`

**Checkpoint**: All user stories should now be independently functional

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T053 [P] Add comprehensive --help documentation for all subcommands and flags
- [X] T054 [P] Run `cargo fmt` across all source files and verify formatting
- [X] T055 Run `cargo clippy` and resolve all warnings and suggestions
- [X] T056 [P] Add end-to-end integration tests for complete CLI workflows in `tests/integration/cli_test.rs`
- [X] T057 Verify quickstart.md with actual command execution and update examples

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can proceed sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) — No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) — Reuses API client and output formatting from US1
- **User Story 3 (P1)**: Can start after Foundational (Phase 2) — Enhances error handling across all commands
- **User Story 4 (P3)**: Can start after Foundational (Phase 2) — Depends on storage module; integrates with US1/US2 via --favorite flag

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Models before services
- Services before command handlers
- Command handlers before main.rs wiring
- Core implementation before integration

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002, T003)
- All Foundational tasks marked [P] can run in parallel (T004, T005, T006, T008, T011, T012, T014)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different developers (after Phase 2)

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit tests for Location validation in tests/unit/models_test.rs"
Task: "Unit tests for WeatherData serialization in tests/unit/models_test.rs"
Task: "Unit tests for text output formatting in tests/unit/output_test.rs"
Task: "Unit tests for JSON output validity in tests/unit/output_test.rs"

# Launch implementation tasks (after tests fail):
Task: "Implement fetch command handler in src/commands/fetch.rs"
Task: "Add clap flags to fetch subcommand in src/main.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP AND VALIDATE**: Run `cargo test` and `weatherfrog fetch --city "London"`
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (fetch current weather)
   - Developer B: User Story 3 (error handling)
   - Developer C: User Story 2 (forecast) after US1 API client is stable
3. After US1-3 complete: Developer A or B tackles User Story 4 (favorites)
4. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- **Constitution requirement**: No `.unwrap()` or `.expect()` calls anywhere in production code
