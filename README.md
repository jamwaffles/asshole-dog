# Asshole Dog 👌

My neighbour's dog won't stop barking, therefore it is an asshole. This is an app for tracking precisely _when_ it was an asshole as some weird form of catharsis.

## Frontend

```bash
cd frontend
cargo web start
```

<http://localhost:8000>

## Backend

* Go to <https://developer.microsoft.com/en-us/graph/graph-explorer#> or something
* Either:
	* Copy the `hello` localstorage key, extract `msft.access_token` and add it to `.env` under `MS_AUTH_TOKEN` OR
	* Make some graph explorer request and copy the `Authorization` header (sans `Bearer ` prefix) into `.env` under `MS_AUTH_TOKEN`

```bash
docker-compose up -d postgres

cd backend
diesel migration run
cargo run
```

<http://localhost:3001/api>