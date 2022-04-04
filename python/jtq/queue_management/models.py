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
