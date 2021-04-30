# Web server

Uses Rocket web framework and Diesel ORM. Rocket requires nightly (`cargo +nightly ...`)

Command for checking how many words are in the table
```
$ mysql -uroot -e "select count(*) from sicniki_sumniki.words"
```

Also contains binary for scraping websites for words (sources are stored in database alongside `words` table).