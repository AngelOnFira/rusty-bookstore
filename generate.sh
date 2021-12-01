#!/bin/bash

rm -rf backend/src/schema/
sea-orm-cli generate entity -o backend/src/schema/ --tables \
    books_book
