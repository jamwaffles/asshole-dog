# Asshole Dog 👌

My neighbour's dog won't stop barking, therefore it is an asshole. This is an app for tracking precisely _when_ it was an asshole as some weird form of catharsis.

## Frontend

```bash
cd frontend
cargo web start
```

<http://localhost:8000>

## Backend

```bash
docker-compose up -d postgres

cd backend
diesel migration run
cargo run
```

<http://localhost:3001/api>