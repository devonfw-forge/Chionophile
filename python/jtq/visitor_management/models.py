from django.db import models

class Visitor(models.Model):
    class Meta():
        db_table = "visitor"

    id = models.AutoField(primary_key=True)
    modificationCounter = models.IntegerField(default=1, db_column="modificationcounter")
    username = models.TextField()
    name = models.TextField()
    password = models.TextField()
    phoneNumber = models.TextField(db_column="phonenumber")
    acceptedCommercial = models.BooleanField(db_column="acceptedcommercial")
    acceptedTerms = models.BooleanField(db_column="acceptedterms")
    userType = models.BooleanField(db_column="usertype")
