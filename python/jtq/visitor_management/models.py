from django.db import models

class Visitor(models.Model):
    class Meta():
        db_table = "visitor"

    id = models.IntegerField(primary_key=True)
    modificationcounter = models.IntegerField()
    username = models.TextField()
    name = models.TextField()
    password = models.TextField()
    phonenumber = models.TextField()
    acceptedcommercial = models.BooleanField()
    acceptedterms = models.BooleanField()
    usertype = models.BooleanField()

