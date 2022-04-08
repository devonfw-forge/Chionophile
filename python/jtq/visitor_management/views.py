from rest_framework import generics, status
from rest_framework.response import Response
from visitor_management.models import Visitor
from visitor_management.serializer import VisitorSerializer

class VisitorDetailView(generics.CreateAPIView, generics.RetrieveAPIView, generics.DestroyAPIView):
    queryset = Visitor.objects.all()
    serializer_class = VisitorSerializer

    def create(self, request, *args, **kwargs):
        response = super().create(request, *args, **kwargs)
        return Response(response.data, status=status.HTTP_200_OK)

    def retrieve(self, request, *args, **kwargs):
        try:
            response = super().retrieve(request, *args, **kwargs)
            return Response(response.data, status=status.HTTP_200_OK)
        except:
            return Response(status=status.HTTP_404_NOT_FOUND)

    def destroy(self, request, pk, *args, **kwargs):
        try:
            response = super().destroy(request, *args, **kwargs)
            return Response(pk, status=status.HTTP_200_OK)
        except:
            return Response(status=status.HTTP_404_NOT_FOUND)

class VisitorListView(generics.ListAPIView):
    serializer_class = VisitorSerializer

    def post(self, request, *args, **kwargs):
        data_query = {}
        for key in self.request.data:
            if key == "pageable":
                pageable = self.request.data['pageable']
            else:
                data_query[key]=self.request.data[key]
                
        if data_query:
            visitors=Visitor.objects.filter(**data_query)
        else:
            visitors=Visitor.objects.all()

        if len(pageable['sort']):
            visitors = visitors.order_by(**pageable['sort'])
            
        pgN = int(pageable['pageNumber'])
        pgS = int(pageable['pageSize'])
        visitors = visitors[pgN*pgS : (1+pgN)*pgS]
        data = VisitorSerializer(visitors, many=True)
        return Response( {"content": data.data, "pageable": pageable, "totalElements": len(data.data)},status=status.HTTP_200_OK)
