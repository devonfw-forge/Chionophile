from email.policy import default
from rest_framework import serializers
from access_code_management.models import AccessCode

def generate_code(number):
        return "Q{}".format(str(number).zfill(3))

class AccessCodeSerializerCTO(serializers.ModelSerializer):
    class Meta:
        model = AccessCode
        fields = '__all__'
        depth=1
    
    def to_representation(self, instance):
        data = super(AccessCodeSerializerCTO, self).to_representation(instance)
        accessCode = {
                        'modificationCounter': data['modificationCounter'],
                        'id': data['id'],
                        'ticketNumber': generate_code(data['id']),
                        'creationTime': data['creationTime'],
                        'starTime': data['startTime'],
                        'visitorId': data['visitorId']['id'],
                        'queueId': data['queueId']['id']
                    }
        return {
                'accessCode': accessCode, 
                'visitor': data['visitorId'],
                'queue': data['queueId'] 
            }


class AccessCodeSerializerETO(serializers.ModelSerializer):
    ticketNumber = serializers.SerializerMethodField()

    class Meta:
        model = AccessCode
        fields = ["modificationCounter", "id", "creationTime", "visitorId", "queueId", "ticketNumber"]

    def get_ticketNumber(self, data):
        return generate_code(data.id)