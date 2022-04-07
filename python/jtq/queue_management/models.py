from django.db import models

class Queue(models.Model):
    class Meta():
        db_table = "dailyqueue"

    id = models.AutoField(primary_key=True, db_column="id")
    modificationCounter = models.IntegerField(db_column="modificationcounter")
    name = models.TextField()
    logo = models.TextField()
    currentNumber = models.TextField(db_column="currentnumber")
    attentionTime = models.DateTimeField(db_column="attentiontime", auto_now_add=True)
    minAttentionTime = models.DateTimeField(db_column="minattentiontime")
    active = models.BooleanField()
