from rest_framework import generics
from rest_framework.response import Response
from rest_framework.decorators import api_view
from rest_framework import status

from access_code_management.models import AccessCode
from access_code_management.serializers import AccessCodeSerializerCTO, AccessCodeSerializerETO

class AccessCodeDetailView(generics.CreateAPIView, generics.RetrieveAPIView):
    queryset = AccessCode.objects.all()
    serializer_class = AccessCodeSerializerCTO

class AccessCodeListView(generics.ListAPIView):
    serializer_class = AccessCodeSerializerCTO

    def post(self, request, *args, **kwargs):
        return super(AccessCodeListView, self).get(request, *args, **kwargs)

@api_view(['POST'])
def searchAccessCode(request):
    pageable = request.data["pageable"]
    if pageable != None:
        pgN = int(pageable['pageNumber'])
        pgS = int(pageable['pageSize'])
        data = AccessCode.objects.all()
        data = AccessCodeSerializerCTO(data[pgN*pgS : (1+pgN)*pgS], many=True)
    else:
        data = AccessCodeSerializerCTO(AccessCode.objects.all(), many=True)
    return Response({"content": data.data, "pageable": pageable, "totalElements": len(data.data) } ,status=status.HTTP_200_OK)


@api_view(['POST'])
def getAccessCodeByVisitorQueue(request):
    if request.data:
        access_code = AccessCodeSerializerETO(data=request.data)
        if access_code.is_valid():
            access_code.save()
            return Response(access_code.data ,status=status.HTTP_201_CREATED) 
        else:
            return Response(access_code.errors, status=status.HTTP_400_BAD_REQUEST)
    else:
        return Response(status=status.HTTP_204_NO_CONTENT)
