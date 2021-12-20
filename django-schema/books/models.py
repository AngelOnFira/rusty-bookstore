from django.db import models

# Design and implement an application for an online bookstore (Look Inna Book).
# This application lets users browse a collection of books that are available in
# the bookstore.
#
# TODO:
# A user can search the bookstore by book name, author name, ISBN, genre, etc..
#
# TODO:
# When a book is selected, information on the author(s), genre, publisher,
# number of pages, price, etc. can be viewed.
#
# TODO:
# A user can select as many books as she likes to be added to the checkout
# basket. A user needs to be registered in the bookstore to be able to checkout.
#
# TODO:
# When checking out, the user inserts billing and shipping information (can be
# different than those used in registration), and completes the order. The
# bookstore has the feature of tracking an order via an order number. A user can
# use this order number to track where the order is currently.
#
# TODO:
# Although shipping is carried out by a third-party shipping service, the online
# bookstore should have the tracking information available for when the user
# inquires about an order using the order number. Assume all books are shipped
# from only one warehouse (no multiple order numbers for multiple books shipped
# from multiple warehouses).
#
# TODO:
# The bookstore owners can add new books to their collections, or remove books
# from their store. They also need to store information on the publishers of
# books such as name, address, email address, phone number(s), banking account,
# etc..
#
# TODO:
# The banking account for publishers is used to transfer a percentage of the
# sales of books published by these publishers. This percentage is variable and
# changes from one book to another.
#
# TODO:
# The owners should have access to reports that show sales vs. expenditures,
# sales per genres, sales per author, etc..
#
# TODO:
# The application should also be able to automatically place orders for new
# books if the remaining quantity is less than a given threshold (e.g., 10
# books). This is done by sending an email to the publisher of the limited books
# to order a number of books equal to how many books were sold in the previous
# month (you do not have to implement the email sending component).

# Steps:

# 1. Create a new Django project.
# 2. Create a new Django app.
# 3. Create a new Django model for each of the following:
#    a. Book
#    b. Author
#    c. Genre
#    d. Publisher
#    e. Order
#    f. OrderLine
#    g. User
#    h. ShippingAddress
#    i. BillingAddress
#    j. Checkout
#    k. Payment
# 4. Create a new Django model for each of the following:
#    a. BookGenre
#    b. BookAuthor
#    c. BookPublisher

# The user can search by:

# 1. Book name
# 2. Author name
# 3. ISBN
# 4. Genre


class Author(models.Model):
    name = models.CharField(max_length=100)
    bio = models.TextField()

    def __str__(self):
        return self.name


class Genre(models.Model):
    name = models.CharField(max_length=100)

    def __str__(self):
        return self.name


class Publisher(models.Model):
    name = models.CharField(max_length=100)

    def __str__(self):
        return self.name


class Book(models.Model):
    name = models.CharField(max_length=100)
    authors = models.ManyToManyField(Author)
    isbn = models.CharField(max_length=100)

    page_count = models.IntegerField(default=0)
    genre = models.ForeignKey(Genre, on_delete=models.CASCADE, null=True)
    publisher = models.ForeignKey(Publisher, on_delete=models.CASCADE, null=True)
    price = models.DecimalField(max_digits=10, decimal_places=2)

    def __str__(self):
        return self.name

class Patron(models.Model):
    name = models.CharField(max_length=100)
    email = models.EmailField()
    phone = models.CharField(max_length=100)
    address = models.CharField(max_length=100)

    def __str__(self):
        return self.name

class Basket(models.Model):
    books = models.ManyToManyField(Book)
    user = models.ForeignKey(Patron, on_delete=models.CASCADE)
    total_price = models.DecimalField(max_digits=10, decimal_places=2)

    def __str__(self):
        return self.user.username

class Order(models.Model):
    user = models.ForeignKey(Patron, on_delete=models.CASCADE)
    order_number = models.IntegerField()

    def __str__(self):
        return self.user.username

# Reduce to a relation schema

# author (id, name, bio)
# genre (id, name)
# publisher (id, name)
# author_book (id, author_id, book_id)
# book (id, name, author_book_id, isbn, page_count, genre_id, publisher_id, price)
# patron (id, name, email, phone, address)
# book_basket (id, book_id, user_id)
# basket (id, book_basket_id, user_id, total_price)
# order (id, user_id, order_number)