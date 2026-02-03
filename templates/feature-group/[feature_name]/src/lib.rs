{% set snake_name = feature_name | snake_case %}

pub use {{ snake_name }}_app as app;
use {{ snake_name }}_infra as infra;
