from rest_framework import generics, status, mixins
from rest_framework.response import Response
import json

from queue_management.models import Queue
from queue_management.serializers import QueueSerializer

class QueueDetailView(generics.UpdateAPIView, mixins.CreateModelMixin, mixins.RetrieveModelMixin,
                         mixins.UpdateModelMixin, mixins.DestroyModelMixin, generics.GenericAPIView,):
    queryset = Queue.objects.all()
    serializer_class = QueueSerializer

    def post(self, request, *args, **kwargs):
        response = super().create(request, *args, **kwargs)
        return Response(response.data, status=status.HTTP_200_OK)
    
    def get(self, request, *args, **kwargs):
        try:
            response = super().retrieve(request, *args, **kwargs)
            return Response(response.data, status=status.HTTP_200_OK)
        except:
            return Response(status=status.HTTP_404_NOT_FOUND)
    
    def delete(self, request, pk, *args, **kwargs):
        try:
            response = super().destroy(request, *args, **kwargs)
            return Response(pk, status=status.HTTP_200_OK)
        except:
            return Response(status=status.HTTP_404_NOT_FOUND)

class QueueListView(generics.ListAPIView):
    serializer_class = QueueSerializer
    def get_queryset(self):
        """
        This view should return a list of all the purchases
        for the currently authenticated user.
        """
        data_query = {}
        for key in self.request.data:
            if key == "pageable":
                pageable = self.request.data['pageable']
            else:
                data_query[key]=self.request.data[key]
                
        if data_query:
            queues=Queue.objects.filter(**data_query)
        else:
            queues=Queue.objects.all()

        if len(pageable['sort']):
            queues = queues.order_by(**pageable['sort'])
        pgN = int(pageable['pageNumber'])
        pgS = int(pageable['pageSize'])
        queues = queues[pgN*pgS : (1+pgN)*pgS]
        return queues

    def post(self, request, *args, **kwargs):
        return super(QueueListView, self).get(request, *args, **kwargs)