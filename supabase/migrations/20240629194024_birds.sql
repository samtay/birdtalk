create table public.birds (
  id integer primary key generated always as identity,
  scientific_name text not null,
  common_name text not null
);

create unique index bird_scientific_name on public.birds using btree (scientific_name);

create table public.bird_images (
  bird integer not null,
  bucket text not null,
  path text not null,
  default_ bool not null default false,
  constraint fk_bird_images_bird
    foreign key (bird) references birds(id)
    on delete cascade,
  constraint fk_bird_images_storage_obj
    foreign key (bucket, path) references storage.objects(bucket_id, name)
    on delete cascade,
  constraint bird_images_pkey primary key (bucket, path)
);
create index idx_bird_images_list on public.bird_images using btree (bird, default_);

create table public.bird_sounds (
  bird integer not null,
  bucket text not null,
  path text not null,
  default_ bool not null default false,
  constraint fk_bird_sounds_bird
    foreign key (bird) references birds(id)
    on delete cascade,
  constraint fk_bird_images_storage_obj
    foreign key (bucket, path) references storage.objects(bucket_id, name)
    on delete cascade,
  constraint bird_sounds_pkey primary key (bucket, path)
);
create index idx_bird_sounds_list on public.bird_sounds using btree (bird, default_);

create table packs (
  id integer primary key generated always as identity,
  name text not null,
  description text not null,
  free bool not null default false
);

create unique index pack_name on public.packs using btree (name);

-- These are easier to create and bulk upload when linking by name, so just do
-- that for now. If perf is a problem, solve it later.
create table bird_pack (
  bird text not null,
  pack text not null,
  constraint fk_bird_pack_bird
    foreign key (bird) references birds(scientific_name)
    on delete cascade,
  constraint fk_bird_pack_pack
    foreign key (pack) references packs(name)
    on delete cascade,
  constraint bird_pack_pkey primary key (bird, pack)
);

create table courses (
  id integer primary key generated always as identity,
  name text not null,
  description text not null,
  free bool not null default false
);

create unique index course_name on public.courses using btree (name);

-- These are easier to create and bulk upload when linking by name, so just do
-- that for now. If perf is a problem, solve it later.
create table course_pack (
  course text not null,
  pack text not null,
  index integer not null,
  constraint fk_course_pack_course
    foreign key (course) references courses(name)
    on delete cascade,
  constraint fk_course_pack_pack
    foreign key (pack) references packs(name)
    on delete cascade,
  constraint course_pack_pkey primary key (course, pack)
);

create unique index course_pack_index on public.course_pack using btree (course, index);

create type sound as (
  path text,
  default_ bool
);
create type bird_detailed as (
  id integer,
  scientific_name text,
  common_name text,
  image text,
  sounds sound[]
);
create view birds_detailed
  with (security_invoker=on)
  as
  select
      b.id,
      b.scientific_name,
      b.common_name,
      bi.bucket || '/' || bi.path as image,
      array_agg((
        bs.bucket || '/' || bs.path,
        bs.default_
      )::sound order by bs.default_) as sounds
  from birds b
  left join bird_images bi on bi.bird = b.id and bi.default_ = true
  left join bird_sounds bs on bs.bird = b.id
  group by b.id, bi.bucket, bi.path;

create type bird_summary as (
  id integer,
  scientific_name text,
  common_name text,
  image text
);
create view birds_summary
  with (security_invoker=on)
  as
  select
      b.id,
      b.scientific_name,
      b.common_name,
      bi.bucket || '/' || bi.path as image
  from birds b
  left join bird_images bi on bi.bird = b.id and bi.default_ = true;

create view bird_packs_summary
  with (security_invoker=on)
  as
  select
      p.id,
      p.name,
      p.description,
      array_agg(b.*) as birds
  from packs p
  left join bird_pack bp on p.name = bp.pack
  left join birds_summary b on b.scientific_name = bp.bird
  group by p.id;

create view bird_packs_detailed
  with (security_invoker=on)
  as
  select
      p.id,
      p.name,
      p.description,
      array_agg(b.*) as birds
  from packs p
  left join bird_pack bp on p.name = bp.pack
  left join birds_detailed b on b.scientific_name = bp.bird
  group by p.id;
