This application only supports REST API calls.

## Test

### Test Command

`RUST_LOG=trace BASE_SPIRA_URL="https://demo-us.spiraservice.net/aimai" SPIRA_USERNAME="administrator" SPIRA_API_KEY="" cargo run --features log --example list_projects`