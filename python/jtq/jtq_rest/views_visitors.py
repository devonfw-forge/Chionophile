from django.http import JsonResponse
from jtq_rest.models import Visitor

def visitor(request):
    qqueue = Visitor.objects.all().values()
    return JsonResponse(list(qqueue), safe=False)