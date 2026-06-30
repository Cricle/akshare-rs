# 100% Interface Verification Design

## Problem Statement

The project has 1412 MCP tool definitions in `tools.txt`, but there's no comprehensive verification that:
1. All tools compile correctly (tools.txt matches client methods)
2. All tools can be called at runtime
3. All parameter schemas are valid

## Solution: Mixed Verification Approach

### Layer 1: Compile-Time Verification (build.rs)

**Goal**: Ensure every tool in `tools.txt` has a matching client method with correct signature.

**Implementation**:
- Parse `tools.txt` to extract tool names and client calls
- In `build.rs`, validate that each client call expression compiles
- Generate a verification function that asserts all tool registrations succeed

**Verification Points**:
- Tool name exists in registry
- Client method exists on `AkShareClient`
- Parameter type matches method signature
- Return type is serializable

### Layer 2: Parameter Validation Tests

**Goal**: Verify all parameter schemas are valid and required fields are enforced.

**Implementation**:
- Generate test for each tool that validates its JSON schema
- Test that required parameters are enforced
- Test that optional parameters have correct defaults

**Test Structure**:
```rust
#[test]
fn test_tool_param_schema_<tool_name>() {
    let schema = get_tool_schema("<tool_name>");
    assert_required_fields(&schema, &["field1", "field2"]);
    assert_optional_fields(&schema, &["field3"]);
}
```

### Layer 3: E2E Tests for Critical Tools

**Goal**: Verify key tools work with real APIs.

**Implementation**:
- Select representative tools from each category (stock, bond, fund, etc.)
- Generate E2E tests with real API calls
- Tests are `#[ignore]` by default, run with `RUN_E2E=1`

**Coverage Target**:
- 1-2 tools per category (stock, bond, fund, futures, option, forex, crypto, macro, news, economy)
- ~20-30 E2E tests total

## Implementation Plan

### Phase 1: Compile-Time Verification

1. **Enhance build.rs**:
   - Parse tools.txt client calls
   - Validate method existence (compile-time check)
   - Generate verification assertions

2. **Add compile-time test**:
   ```rust
   #[test]
   fn test_all_tools_compile() {
       // This test passes if build.rs generated valid code
       assert!(TOOL_REGISTRY.len() >= 1400);
   }
   ```

### Phase 2: Parameter Validation Tests

1. **Create param test generator**:
   - Read tools.txt
   - For each tool, generate schema validation test
   - Output to `tests/param_validation.rs`

2. **Add test runner**:
   ```bash
   cargo test -p akshare-mcp --test param_validation
   ```

### Phase 3: E2E Test Expansion

1. **Expand e2e.rs**:
   - Add 1-2 E2E tests per category (stock, bond, fund, futures, option, forex, crypto, macro, news, economy)
   - Cover both simple (no-param) and complex (multi-param) tools
   - Keep rate limiting (2s sleep between tests)

## Success Criteria

1. **Compile-Time**: `cargo build` succeeds with zero tool-related errors
2. **Parameter Tests**: All 1412 tools have valid schemas
3. **E2E Tests**: At least 1 tool per category passes E2E test

## File Changes

### Modified Files
- `crates/akshare-mcp/build.rs` - Add compile-time verification and param test generation
- `crates/akshare/tests/e2e.rs` - Expand E2E coverage for all categories

### New Files
- `crates/akshare-mcp/tests/param_validation.rs` - Generated parameter validation tests

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| E2E tests fail due to API changes | Make E2E tests `#[ignore]` by default; run separately |
| Build time increases | Parameter tests are generated, not computed at build time |
| External API rate limits | Add 2s sleep between E2E tests |

## Design Decisions

1. **Parameter test generation**: Generate at build time via build.rs to keep tests in sync with tools.txt
2. **E2E test coverage**: 1-2 tools per category (10-20 tests total) is sufficient
3. **CI integration**: E2E tests run on schedule (weekly), not on every PR
