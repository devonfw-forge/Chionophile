from django.urls import path
import visitor_management.views as visitor_views

urlpatterns = [
    path(
        '', 
        visitor_views.VisitorDetailView.as_view()
    ),
    path(
        '<int:pk>', 
        visitor_views.VisitorDetailView.as_view()
    ),
    path(
        'search', 
        visitor_views.VisitorListView.as_view()
    ),
]