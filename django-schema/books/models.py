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

class Book(models.Model):
    name = models.CharField(max_length=100)
    isbn = models.CharField(max_length=100)
    price = models.DecimalField(max_digits=10, decimal_places=2)
    quantity = models.IntegerField()
    description = models.TextField()

    def __str__(self):
        return self.name

class Author(models.Model):
    name = models.CharField(max_length=100)
    bio = models.TextField()

    def __str__(self):
        return self.name

# class Basket(models.Model):
#     user = models.ForeignKey(User, on_delete=models.CASCADE)
#     books = models.ManyToManyField(Book)
#     quantity = models.IntegerField()

#     def __str__(self):
#         return self.book.name

# class Order(models.Model):
#     user = models.ForeignKey(User, on_delete=models.CASCADE)
#     order_number = models.IntegerField()
#     date = models.DateField()
#     shipping_address = models.ForeignKey(ShippingAddress, on_delete=models.CASCADE)
#     billing_address = models.ForeignKey(BillingAddress, on_delete=models.CASCADE)
#     payment = models.ForeignKey(Payment, on_delete=models.CASCADE)

#     def __str__(self):
#         return self.order_number