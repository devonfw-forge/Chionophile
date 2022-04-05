from rest_framework import generics
import json

from queue_management.models import Queue
from queue_management.serializers import QueueSerializer

class QueueDetailView(generics.CreateAPIView, generics.RetrieveAPIView):
    queryset = Queue.objects.all()
    serializer_class = QueueSerializer

class QueueListView(generics.ListAPIView):
    serializer_class = QueueSerializer
    def get_queryset(self):
        """
        This view should return a list of all the purchases
        for the currently authenticated user.
        """
        requests = self.request
        search = json.loads(requests.body)
        pageable = search['pageable']
        queues = None
        if 'active' in search:
            status_queue = search['active']
            queues=Queue.objects.filter(active=status_queue)
        if queues == None:
            queues=Queue.objects.all()
        if len(pageable['sort']):
            queues = queues.order_by(**pageable['sort'])
        pgN = int(pageable['pageNumber'])
        pgS = int(pageable['pageSize'])
        queues = queues[pgN*pgS : (1+pgN)*pgS]
        return queues

    def post(self, request, *args, **kwargs):
        return super(QueueListView, self).get(request, *args, **kwargs)