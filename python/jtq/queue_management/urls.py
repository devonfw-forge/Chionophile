from django.urls import path
import queue_management.views as queue_views

urlpatterns = [
    path(
        '', 
        queue_views.QueueDetailView.as_view()
    ),
    path(
        '<int:pk>', 
        queue_views.QueueDetailView.as_view()
    ),
    path(
        '<int:pk>/', 
        queue_views.QueueDetailView.as_view()
    ),
    path(
        'search/', 
        queue_views.QueueListView.as_view()
    ),
]