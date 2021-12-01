#!/bin/bash

rm -rf app/src/schema/
sea-orm-cli generate entity -o app/src/schema/ --tables \
    books_book
