from django.http import JsonResponse
from jtq_rest.models import Queue

def queue(request):
    qqueue = Queue.objects.all().values()
    return JsonResponse(list(qqueue), safe=False)