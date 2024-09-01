create table public.stats (
  user_id uuid primary key,
  data jsonb not null default '{}',
  updated_at timestamp with time zone not null default (now() at time zone 'utc'),
  constraint fk_stats_users_user_id
    foreign key (user_id) references auth.users(id)
    on delete cascade
);

-- TODO: we'll add an incremental version # here to facilitate merging local/remote
create or replace function upsert_stats(user_id uuid, data jsonb, updated_at timestamptz)
returns void
language plpgsql
as $$
#variable_conflict use_column
begin
  insert into stats(user_id, data, updated_at)
  values (upsert_stats.user_id, upsert_stats.data, upsert_stats.updated_at)
  on conflict (user_id)
  do update set data = upsert_stats.data, updated_at = upsert_stats.updated_at;
end;
$$;
