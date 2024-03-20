Tokio Postgres Dynamic Query Example
====================================
This is an example project showing how to create dynamic queries for the rust tokio postgres library.  To start run
```
docker-compose up -d
```
to start the db server.

Dbmate was used to make the migrations but you could copy up part of the migration code in the `migrations/db/migrations/20240320053819_init.sql` file to create the db.
```
dbmate migrate
```
Insert the data in the `insert.sql` file then run
```
cargo run -- --help
```
to see all the query options and mix and match.  For example, running
```
cargo run -- --money-in-bank-gt 40 --money-in-bank-lt 80
```
Should return
```
select * from items where money_in_bank < $1 and money_in_bank > $2 
Item { id: 1, name: "buckey", money_in_bank: 60 }
Item { id: 2, name: "becky", money_in_bank: 50 }
```
