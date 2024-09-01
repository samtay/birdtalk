-- Drop these for now
drop table "course_pack";
drop table "courses";

-- Enable RLS
alter table "birds" enable row level security;
alter table "bird_images" enable row level security;
alter table "bird_sounds" enable row level security;
alter table "bird_pack" enable row level security;
alter table "packs" enable row level security;
alter table "daily_packs" enable row level security;
alter table "stats" enable row level security;

-- Enable read access for public bird data
create policy "Enable read access for all users"
on "public"."birds"
as PERMISSIVE
for SELECT
to public
using (true);

create policy "Enable read access for all users"
on "public"."bird_images"
as PERMISSIVE
for SELECT
to public
using (true);

create policy "Enable read access for all users"
on "public"."bird_sounds"
as PERMISSIVE
for SELECT
to public
using (true);

create policy "Enable read access for all users"
on "public"."bird_pack"
as PERMISSIVE
for SELECT
to public
using (true);

create policy "Enable read access for all users"
on "public"."packs"
as PERMISSIVE
for SELECT
to public
using (true);

create policy "Enable read access for all users"
on "public"."daily_packs"
as PERMISSIVE
for SELECT
to public
using (true);

-- Enable CRUD access for stats table
create policy "Enable read access based on user_id"
on "public"."stats"
as PERMISSIVE
for ALL
to authenticated
using ( (select auth.uid()) = user_id )
with check ( (select auth.uid()) = user_id );

-- Fix mutable search paths
create or replace function upsert_stats(user_id uuid, data jsonb, updated_at timestamptz)
returns void
language plpgsql
set search_path = ''
as $$
#variable_conflict use_column
begin
  insert into public.stats(user_id, data, updated_at)
  values (upsert_stats.user_id, upsert_stats.data, upsert_stats.updated_at)
  on conflict (user_id)
  do update set data = upsert_stats.data, updated_at = upsert_stats.updated_at;
end;
$$;

create or replace function create_random_daily_pack(day date)
returns bigint
language plpgsql
set search_path = ''
as $$
declare
  new_pack_id bigint;
begin
  insert into public.packs(name, description, free)
  values (
    'Daily Bevy for ' || create_random_daily_pack.day,
    'The daily challenge for ' || to_char(create_random_daily_pack.day, 'FMDay, FMMonth FMDDth YYYY'),
    true
  )
  returning id into new_pack_id;

  insert into public.daily_packs(pack, day)
  values (new_pack_id, create_random_daily_pack.day);

  insert into public.bird_pack(bird, pack)
  select
    bs.scientific_name, 'Daily Bevy for ' || create_random_daily_pack.day
  from (
    -- for now let's keep this deterministic
    select pg_catalog.setseed((create_random_daily_pack.day - '2024-01-01') / 50000.0) as seed, null as scientific_name
    union all
    select null as seed, scientific_name from public.birds
    offset 1
  ) bs
  order by pg_catalog.random()
  limit 10;

  return new_pack_id;
end;
$$;
