create extension if not exists "uuid-ossp";

create table timestamps (
	id uuid primary key default uuid_generate_v4(),
	created_at timestamptz not null default now()
);