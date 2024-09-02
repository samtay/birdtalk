-- allow packs to just be called "Daily Bevy"

-- enable cron extension
create extension if not exists pg_cron with schema extensions;
grant usage on schema cron to postgres;
grant all privileges on all tables in schema cron to postgres;

-- create table for pack of the day
create table daily_packs(
  pack integer primary key references packs(id),
  day date not null
);
create unique index daily_packs_day on daily_packs using btree (day);

-- create fn to populate daily pack
create or replace function create_random_daily_pack(day date)
returns bigint
language plpgsql
as $$
declare
  new_pack_id bigint;
begin
  insert into packs(name, description, free)
  values (
    'Daily Bevy for ' || create_random_daily_pack.day,
    'The daily challenge for ' || to_char(create_random_daily_pack.day, 'FMDay, FMMonth FMDDth YYYY'),
    true
  )
  returning id into new_pack_id;

  insert into daily_packs(pack, day)
  values (new_pack_id, create_random_daily_pack.day);

  insert into bird_pack(bird, pack)
  select
    bs.scientific_name, 'Daily Bevy for ' || create_random_daily_pack.day
  from (
    -- for now let's keep this deterministic
    select setseed((create_random_daily_pack.day - '2024-01-01') / 50000.0) as seed, null as scientific_name
    union all
    select null as seed, scientific_name from birds
    offset 1
  ) bs
  order by random()
  limit 10;

  return new_pack_id;
end;
$$;

-- Might as well insert today's pack (which might be yesterday/tomorrow in UTC)
select create_random_daily_pack(current_date - 1);
select create_random_daily_pack(current_date);
select create_random_daily_pack(current_date + 1);

select cron.schedule(
  'next-pack-of-the-day',
  '1 0 * * *', -- everyday, just after midnight
  $$
  -- create tomorrow's pack if it doesn't exist yet
  select create_random_daily_pack(current_date + 1)
  where not exists (select from daily_packs where day = current_date + 1);
  $$
);

