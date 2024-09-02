-- drop dependent views
drop view bird_packs_summary;
drop view bird_packs_detailed;

-- create new table, populate it, delete the old one
create table bird_pack_v2 (
  bird integer not null,
  pack integer not null,
  constraint fk_bird_pack_bird
    foreign key (bird) references birds(id)
    on delete cascade,
  constraint fk_bird_pack_pack
    foreign key (pack) references packs(id)
    on delete cascade,
  constraint bird_pack_link_pkey primary key (bird, pack)
);

insert into bird_pack_v2(bird, pack)
select
  birds.id, packs.id
from bird_pack
  inner join birds on birds.scientific_name = bird_pack.bird
  inner join packs on packs.name = bird_pack.pack;

drop table bird_pack;
alter table bird_pack_v2 rename to bird_pack;

-- renable rls
alter table "bird_pack" enable row level security;
create policy "Enable read access for all users"
on "public"."bird_pack"
as PERMISSIVE
for SELECT
to public
using (true);

-- recreate views
create view bird_packs_detailed
  with (security_invoker=on)
  as
  select
      p.id,
      p.name,
      p.description,
      array_agg(b.*) as birds,
      dp.day
  from packs p
  left join bird_pack bp on p.id = bp.pack
  left join birds_detailed b on b.id = bp.bird
  left join daily_packs dp on dp.pack = p.id
  group by p.id, dp.day;

-- recreate functions
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
    bs.bird_id, new_pack_id
  from (
    -- for now let's keep this deterministic
    select pg_catalog.setseed((create_random_daily_pack.day - '2024-01-01') / 50000.0) as seed, null as bird_id
    union all
    select null as seed, id as bird_id from public.birds
    offset 1
  ) bs
  order by pg_catalog.random()
  limit 10;

  return new_pack_id;
end;
$$;
