from django.db import models
from visitor_management.models import Visitor
from queue_management.models import Queue

class AccessCode(models.Model):
    class Meta():
        db_table = "accesscode"

    id = models.IntegerField(primary_key=True)
    modificationCounter = models.IntegerField(db_column="modificationcounter")
    ticketNumber = models.TextField(db_column="ticketnumber")
    creationTime = models.DateField(db_column="creationtime")
    startTime = models.DateField(db_column="starttime")
    endTime = models.DateField(db_column="endtime")
    visitor = models.ForeignKey(Visitor, on_delete=models.SET_NULL, null=True, db_column="idvisitor")
    queue = models.ForeignKey(Queue, on_delete=models.SET_NULL, null=True, db_column="idqueue")