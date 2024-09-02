create or replace view bird_packs_detailed
  with (security_invoker=on)
  as
  select
      p.id,
      p.name,
      p.description,
      array_agg(b.*) as birds,
      dp.day
  from packs p
  left join bird_pack bp on p.name = bp.pack
  left join birds_detailed b on b.scientific_name = bp.bird
  left join daily_packs dp on dp.pack = p.id
  group by p.id, dp.day;
