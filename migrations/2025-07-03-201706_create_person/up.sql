create table person (
    person_id serial primary key,
    first_name text,
    last_name text not null,
    created_at timestamptz not null default now()
);
