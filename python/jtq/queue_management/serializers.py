from rest_framework import serializers
from queue_management.models import Queue

class QueueSerializer(serializers.ModelSerializer):
    class Meta:
        model = Queue
        fields = '__all__'