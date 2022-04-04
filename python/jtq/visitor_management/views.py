from copy import error
from django.http import request
from django.http.response import HttpResponseNotAllowed
from django.views.decorators.csrf import csrf_exempt
from rest_framework import serializers, views, generics
from rest_framework.response import Response
from http import HTTPStatus
import json

from visitor_management.models import Visitor
from visitor_management.serializer import VisitorSerializer

class VisitorDetailView(generics.CreateAPIView, generics.RetrieveAPIView):
    queryset = Visitor.objects.all()
    serializer_class = VisitorSerializer

class VisitorListView(generics.ListAPIView):
    serializer_class = VisitorSerializer
    def get_queryset(self):
        """
        This view should return a list of all the purchases
        for the currently authenticated user.
        """
        requests = self.request
        search = json.loads(requests.body)
        pageable = search['pageable']
        visitors = None
        if 'username' in search:
            user_name = search['username']
            visitors=Visitor.objects.filter(username=user_name)
        if visitors == None:
            visitors=Visitor.objects.all()
        if len(pageable['sort']):
            visitors = visitors.order_by(**pageable['sort'])
        pgN = int(pageable['pageNumber'])
        pgS = int(pageable['pageSize'])
        print(pgN*pgS)
        print((1+pgN)*pgS)
        visitors = visitors[pgN*pgS : (1+pgN)*pgS]
        return visitors

    def post(self, request, *args, **kwargs):
        return super(VisitorListView, self).get(request, *args, **kwargs)
