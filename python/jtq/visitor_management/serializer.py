from rest_framework import serializers
from visitor_management.models import Visitor

class VisitorSerializer(serializers.ModelSerializer):
    class Meta:
        model = Visitor
        fields = '__all__'

    def create(self, validated_data):
        return super(VisitorSerializer, self).create(validated_data)
    
    def update(self, instance, validated_data):
        super(VisitorSerializer, self).update(instance, validated_data)
        return instance