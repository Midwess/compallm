# Testing

Write tests only in `tests/` directories, using the existing `test/tests/` suite for shared Rust tests. Do not add tests under `api/` or inline in application source. Test the application's core business logic; do not write tests that only verify dependency or framework behavior.

Do not write unit tests or automated browser/interaction test suites for `libs/storybook`. Do not recreate `libs/storybook/tests` or add test runners there. Validate the library with lint, type checks, builds, and manual Storybook inspection. Keep tests for consumer application business logic in the consuming application's `tests/` directory.

Unit test will take a lot of effort to maintains. Only write important, main flow tests or critical edge case tests.

Important rule is every function is limit 0-2 unit tests.

Focus on qualified each unit test function not test non-sense cases focus on main use case of function.

Never test string, logs, print content, or error message, never test input validations too
