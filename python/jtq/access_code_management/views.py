from rest_framework import generics
from rest_framework.response import Response
from rest_framework.decorators import api_view
from rest_framework import status

from access_code_management.models import AccessCode
from access_code_management.serializers import AccessCodeSerializerCTO, AccessCodeSerializerETO

class AccessCodeDetailView(generics.CreateAPIView, generics.RetrieveAPIView, generics.DestroyAPIView):
    queryset = AccessCode.objects.all()
    serializer_class = AccessCodeSerializerCTO

    def retrieve(self, request, *args, **kwargs):
        try:
            response = super().retrieve(request, *args, **kwargs)
            return Response(response.data, status=status.HTTP_200_OK)
        except:
            return Response(status=status.HTTP_404_NOT_FOUND)

    def post(self, request, *args, **kwargs):
        if request.data:
            access_code = AccessCodeSerializerETO(data=request.data)
            if access_code.is_valid():
                access_code.save()
                return Response(access_code.data ,status=status.HTTP_200_OK) 
            else:
                return Response(access_code.errors, status=status.HTTP_400_BAD_REQUEST)
        else:
            return Response(status=status.HTTP_204_NO_CONTENT)

    def destroy(self, request, pk, *args, **kwargs):
        try:
            response = super().destroy(request, *args, **kwargs)
            return Response(pk, status=status.HTTP_200_OK)
        except:
            return Response(status=status.HTTP_404_NOT_FOUND)

class AccessCodeListView(generics.ListAPIView):
    serializer_class = AccessCodeSerializerCTO

    def post(self, request, *args, **kwargs):
        return super(AccessCodeListView, self).get(request, *args, **kwargs)

@api_view(['POST'])
def searchAccessCode(request):
    data_query = {}
    for key in request.data:
        if key == "pageable":
            pageable = request.data['pageable']
        elif key == "ticketNumber":
            data_query["id"]=int(request.data[key][1:])
        else:
            data_query[key]=request.data[key]
            
    if data_query:
        accessCode=AccessCode.objects.filter(**data_query)
    else:
        accessCode=AccessCode.objects.all()
    
    if len(pageable['sort']):
        accessCode = accessCode.order_by(**pageable['sort'])

    
    pgN = int(pageable['pageNumber'])
    pgS = int(pageable['pageSize'])
    accessCode = AccessCodeSerializerCTO(accessCode[pgN*pgS : (1+pgN)*pgS], many=True)
    return Response({"content": accessCode.data, "pageable": pageable, "totalElements": len(accessCode.data) } ,status=status.HTTP_200_OK)
