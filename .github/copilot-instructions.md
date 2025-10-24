# Copilot Instructions for Rants

## Repository Overview

**Rants** is an async NATS client library for Rust. It provides an ergonomic, thin wrapper over the NATS client protocol with TLS support powered by either `native-tls` or `rustls`.

## Repository Structure

```
/
├── Cargo.toml              # Main project configuration with dependencies
├── Cargo.lock              # Dependency lock file
├── README.md               # Project documentation
├── LICENSE-APACHE          # Apache license
├── LICENSE-MIT             # MIT license
├── CHANGELOG.md            # Version history
├── src/                    # Main source code
│   ├── lib.rs              # Library entry point and main Client implementation
│   ├── codec.rs            # NATS protocol codec implementation
│   ├── tls_or_tcp_stream.rs # TLS/TCP stream abstraction
│   ├── types.rs            # Core types and structures
│   ├── util.rs             # Utility functions and constants
│   ├── tests.rs            # Unit tests for lib.rs
│   ├── codec/              # Codec-specific modules
│   │   └── tests.rs        # Codec unit tests
│   └── types/              # Type-specific modules
│       ├── address.rs      # Address handling
│       ├── error.rs        # Error types
│       ├── parser.rs       # Protocol parsing
│       ├── refs.rs         # Reference types
│       ├── state.rs        # Connection state management
│       ├── tls.rs          # TLS configuration
│       ├── tests.rs        # Type tests
│       └── parser/         # Parser-specific modules
│           └── tests.rs    # Parser unit tests
├── tests/                  # Integration tests
│   ├── authorization_override.rs
│   ├── blocking.rs
│   ├── common.rs
│   ├── echo.rs
│   ├── native_tls_connection.rs
│   ├── ping_pong.rs
│   ├── reconnect.rs
│   ├── request.rs
│   ├── rustls_tls_connection.rs
│   ├── wild_card_subject.rs
│   └── certs/              # Test certificates
│       ├── ca.pem
│       ├── server-crt.pem
│       └── server-key.pem
└── target/                 # Build artifacts (gitignored)
```

## Critical Instructions

### File Modification Restrictions
- **DO NOT MODIFY** any `*.codegen.go` files if they are present in the repository
- These files are auto-generated and should never be manually edited

### Development Workflow

When implementing tasks, follow this comprehensive workflow:

#### 1. Task Analysis and Planning
- Read and understand the complete task requirements
- Break down complex tasks into smaller, manageable components
- Identify which files and modules will need modification
- Plan the implementation approach before making any changes

#### 2. Jira Integration (when Jira ID provided)
- Use the **atlassian-mcp-server** MCP server to fetch Jira issue details
- Read the complete story description and acceptance criteria
- Understand the business context and requirements
- Align implementation with the Jira story requirements

#### 3. Implementation
- Make incremental changes with clear, logical commits
- Follow Rust best practices and idioms
- Ensure code follows the existing patterns in the codebase
- Maintain backward compatibility unless explicitly required to break it
- Update documentation (inline comments and README if needed)

#### 4. Testing Requirements
- **MANDATORY**: Create comprehensive unit test cases for all implementations
- **MANDATORY**: Maintain code coverage > 80% at all times
- Run existing tests to ensure no regressions: `cargo test`
- For integration tests requiring NATS server, ensure `NATS_PATH` environment variable points to NATS server executable
- Use `env_logger` for debugging: `RUST_LOG=rants=trace cargo test <test_name>`
- Test both happy path and edge cases
- Include tests for error conditions

#### 5. Code Quality Checks
- Run `cargo fmt` to format code
- Run `cargo clippy` to catch common mistakes
- Ensure all compiler warnings are addressed
- Verify documentation builds: `cargo doc`

#### 6. Branch Creation and PR Workflow (when requested)
- Use the Jira ID as the branch name (e.g., `PROJ-123`)
- Create branch: `gh repo clone <repo> && cd <repo> && git checkout -b <jiraId>`
- Make and commit changes with descriptive commit messages
- Push changes: `git push origin <jiraId>`
- Create PR using GitHub CLI: `gh pr create --title "<jiraId>: <summary>" --body "<html_formatted_description>"`
- Add label "runtest:all:stable" to the PR: `gh pr edit <pr_number> --add-label "runtest:all:stable"`
- Add label "ai-assisted" to the PR: `gh pr edit <pr_number> --add-label "ai-assisted"`

#### PR Description Format
Use HTML tags for formatting the PR description:
```html
<h2>Summary</h2>
<p>Brief description of changes made</p>

<h2>Changes Made</h2>
<ul>
<li>Specific change 1</li>
<li>Specific change 2</li>
</ul>

<h2>Testing</h2>
<p>Description of tests added/modified and coverage impact</p>

<h2>AI Assistance</h2>
<p>This work was completed with AI assistance following Progress AI policies</p>

<h2>Jira</h2>
<p>Relates to: <a href="[jira_url]">[JiraID]</a></p>
```

### Prompt-Based Workflow
- **ALL tasks must be prompt-based**
- After each significant step, provide a summary of what was completed
- Clearly state what the next step will be
- List remaining steps to completion
- **ALWAYS ASK**: "Do you want to continue with the next step?" before proceeding
- Wait for explicit confirmation before continuing

### Project-Specific Guidelines

#### Rust/Cargo Specifics
- Follow Rust 2018 edition conventions
- Use `tokio` for async runtime (already configured)
- Respect the optional feature flags: `native-tls`, `rustls-tls`, `tls`
- Maintain compatibility with specified dependency versions
- Use `log` crate for logging, not `println!`

#### NATS Protocol
- Understand NATS protocol basics before making protocol-related changes
- Respect message size limits and server capabilities
- Handle connection states properly (Connected, Connecting, Disconnected, Disconnecting)
- Implement proper reconnection logic

#### Error Handling
- Use the custom `Error` type defined in `types/error.rs`
- Provide meaningful error messages
- Handle network failures gracefully
- Implement proper timeout handling

#### Performance Considerations
- Avoid blocking operations in async contexts
- Use appropriate buffer sizes for channels
- Minimize memory allocations in hot paths
- Consider connection pooling and reuse

### Testing Strategy
1. **Unit Tests**: Test individual functions and methods in isolation
2. **Integration Tests**: Test complete workflows with real NATS server
3. **Error Path Testing**: Verify proper error handling and recovery
4. **Concurrent Testing**: Test thread safety and async behavior
5. **TLS Testing**: Verify both native-tls and rustls-tls features
6. **Performance Testing**: Benchmark critical paths where applicable

### Debugging Guidelines
- Use `trace!`, `debug!`, `info!`, `warn!`, `error!` macros appropriately
- Enable logging for debugging: `RUST_LOG=rants=trace`
- Use `debug_assert!` for development-time checks
- Provide context in error messages

## Step-by-Step Implementation Process

When implementing any task:

1. **Understand** ➜ Analyze requirements and existing code
2. **Plan** ➜ Design the implementation approach
3. **Implement** ➜ Write the code following best practices
4. **Test** ➜ Create comprehensive tests ensuring >80% coverage
5. **Verify** ➜ Run all tests and quality checks
6. **Document** ➜ Update relevant documentation
7. **Review** ➜ Self-review for completeness and quality
8. **Branch & PR** ➜ Create branch, commit, and PR (if requested)
9. **Update JIRA Ticket** ➜ **MANDATORY**: Update JIRA ticket after successful PR creation

### JIRA Ticket Update Process
After successful PR creation, **MANDATORY** step to update JIRA ticket:
- Use the **atlassian-mcp-server** MCP server to update the JIRA ticket programmatically
- Update custom field `customfield_11170` ("Does this Work Include AI Assisted Code?") to "Yes"
- **CRITICAL**: Use correct field format: `{"customfield_11170": {"value": "Yes"}}`
- Verify the field update was successful
- This step ensures proper tracking of AI-assisted work for compliance and reporting

## Final Notes
- This is a production-grade library used by other Rust projects
- Changes must be backward compatible unless explicitly stated
- Performance and reliability are critical
- Follow semantic versioning principles
- All public APIs must be properly documented
- Consider the impact on both sync and async users of the library

Remember: Quality over speed. It's better to implement fewer features correctly than many features incorrectly.