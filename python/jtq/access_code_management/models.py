from django.db import models
from visitor_management.models import Visitor
from queue_management.models import Queue

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