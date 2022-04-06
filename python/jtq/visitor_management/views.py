from rest_framework import generics, status
from rest_framework.response import Response
from visitor_management.models import Visitor
from visitor_management.serializer import VisitorSerializer

class VisitorDetailView(generics.CreateAPIView, generics.RetrieveAPIView, generics.DestroyAPIView):
    queryset = Visitor.objects.all()
    serializer_class = VisitorSerializer

class VisitorListView(generics.ListAPIView):
    serializer_class = VisitorSerializer

    def post(self, request, *args, **kwargs):
        pageable = self.request.data['pageable']
        visitors = None
        if 'username' in self.request.data:
            user_name = self.request.data['username']
            visitors=Visitor.objects.filter(username=user_name)
        if visitors == None:
            visitors=Visitor.objects.all()
        if len(pageable['sort']):
            visitors = visitors.order_by(**pageable['sort'])
        pgN = int(pageable['pageNumber'])
        pgS = int(pageable['pageSize'])
        visitors = visitors[pgN*pgS : (1+pgN)*pgS]
        data = VisitorSerializer(visitors, many=True)
        return Response( {"content": data.data, "pageable": pageable, "totalElements": len(data.data)},status=status.HTTP_200_OK)
