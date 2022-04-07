from django.db import models
from visitor_management.models import Visitor
from queue_management.models import Queue

class AccessCode(models.Model):
    class Meta():
        db_table = "accesscode"

    id = models.AutoField(primary_key=True)
    modificationCounter = models.IntegerField(db_column="modificationcounter", default=1)
    creationTime = models.DateTimeField(auto_now_add=True, db_column="creationtime")
    startTime = models.DateTimeField(db_column="starttime")
    endTime = models.DateTimeField(db_column="endtime")
    visitorId = models.ForeignKey(Visitor, on_delete=models.SET_NULL, null=True, db_column="idvisitor")
    queueId = models.ForeignKey(Queue, on_delete=models.SET_NULL, null=True, db_column="idqueue")