from telnetlib import LOGOUT
from unicodedata import name
from django.db import models

class Queue(models.Model):
    class Meta():
        db_table = "dailyqueue"

    id = models.IntegerField(primary_key=True)
    modificationcounter = models.IntegerField()
    name = models.TextField()
    logo = models.TextField()
    currentnumber = models.TextField()
    attentiontime = models.DateField()
    minattentiontime = models.DateField()
    active = models.BooleanField()

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

class AccessCode(models.Model):
    class Meta():
        db_table = "accesscode"

    id = models.IntegerField(primary_key=True)
    modificationcounter = models.IntegerField()
    ticketnumber = models.TextField()
    creationtime = models.DateField()
    starttime = models.DateField()
    endtime = models.DateField()
    visitor = models.ForeignKey(Visitor, on_delete=models.SET_NULL, null=True, db_column="idvisitor")
    queue = models.ForeignKey(Queue, on_delete=models.SET_NULL, null=True, db_column="idqueue")