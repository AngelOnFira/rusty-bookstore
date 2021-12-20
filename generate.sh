#!/bin/bash

    # --expanded-format \
    
rm -rf rusty-bookstore-schema/src/schema/
sea-orm-cli generate entity \
    -o rusty-bookstore-schema/src/schema/ \
    --with-serde both \
    --tables \
        books_book,books_author,books_genre,books_publisher,books_patron,books_basket,books_order
