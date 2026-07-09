-- the migration that runs the initial create_random_daily_pack() can create
-- empty packs on an unseeded db. this function fixes that.
create or replace function ensure_daily_pack(day date)
returns void
language plpgsql
as $$
declare
  target_pack_id integer;
  bird_link_count integer;
begin
  select pack into target_pack_id from daily_packs where daily_packs.day = ensure_daily_pack.day;

  if target_pack_id is not null then
    select count(*) into bird_link_count from bird_pack where pack = target_pack_id;
    if bird_link_count = 0 then
      delete from daily_packs where pack = target_pack_id;
      delete from packs where id = target_pack_id;
      target_pack_id := null;
    end if;
  end if;

  if target_pack_id is null then
    perform create_random_daily_pack(ensure_daily_pack.day);
  end if;
end;
$$;
