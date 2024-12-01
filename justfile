set export

DATABASE_URL := "postgres://challengr:sandcastle@localhost/challengr"

# watch and run tests
watch:
    DATABASE_URL="postgres://postgres:postgres@localhost/challengr" cargo watch -x test

# run tests with coverage reporting
coverage:
    DATABASE_URL="postgres://postgres:postgres@localhost/challengr" cargo tarpaulin

# build the docker images
docker-build:
    docker build . -f challengr-api/Dockerfile

# bring up everything in the stack but the app
dev +CMD:
    docker compose --profile=dev {{ CMD }}

# bring up the entire local stack (with containerized app)
full +CMD:
    docker compose --profile=full {{ CMD }}

# run a sqlx migrate command
migrate +CMD:
    cd challengr-api; sqlx migrate {{ CMD }}

# run a sqlx database command
db +CMD:
    cd challengr-api; sqlx database {{ CMD }}
