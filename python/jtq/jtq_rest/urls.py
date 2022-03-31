from django.urls import path

from . import views_access_code
from . import views_queue
from . import views_visitors

urlpatterns = [
    path('queue', views_queue.queue),
    path('visitor', views_visitors.visitor),
    path('accesscode', views_access_code.accesscode),
]