from rest_framework import serializers
from access_code_management.models import AccessCode

class AccessCodeSerializer(serializers.ModelSerializer):
    class Meta:
        fields = '__all__'