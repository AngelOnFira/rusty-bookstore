from django.core.management.base import BaseCommand
from django_seed import Seed
from django.conf import settings

from books.models import Book


class Command(BaseCommand):
    help = "Seed the database with boks"

    def handle(self, *args, **kwargs):
        # Make sure we are in debug mode
        if settings.DEBUG != True:
            return

        Book.objects.all().delete()

        seeder = Seed.seeder(locale="en_CA")

        seeder.add_entity(Book, 10)

        seeder.execute()
