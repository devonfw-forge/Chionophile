from django.http import JsonResponse
from jtq_rest.models import AccessCode

def accesscode(request):
    qqueue = AccessCode.objects.all().values()
    return JsonResponse(list(qqueue), safe=False)