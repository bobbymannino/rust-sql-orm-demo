create table post (
    post_id uuid primary key default gen_random_uuid(),
    title text not null,
    slug text not null unique,
    content text not null,
    author_id int not null references person(person_id),
    created_at timestamptz not null default now()
);
