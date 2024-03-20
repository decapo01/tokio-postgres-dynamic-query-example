-- migrate:up
create table if not exists items (
  id serial PRIMARY key,
  name text not null,
  money_in_bank integer -- just for demo, never use int for actual money
);

-- migrate:down
drop table items;