web: ROCKET_PORT=$PORT ROCKET_ENV=prod ./target/release/weblogue
./target/release/diesel migration revert
./target/release/diesel migration run
