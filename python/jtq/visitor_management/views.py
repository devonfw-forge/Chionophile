from rest_framework import views, generics
from visitor_management.models import Visitor
from visitor_management.serializer import VisitorSerializer

class VisitorDetailView(generics.RetrieveUpdateDestroyAPIView):
    queryset = Visitor.objects.all()
    serializer_class = VisitorSerializer

class VisitorListView(views.APIView):
    pass