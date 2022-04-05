from django.urls import path
import access_code_management.views as access_code_views

urlpatterns = [
    path(
        '', 
        access_code_views.getAccessCodeByVisitorQueue
    ),
    path(
        '<int:pk>', 
        access_code_views.AccessCodeDetailView.as_view()
    ),
    path(
        'cto/search', 
        access_code_views.searchAccessCode
    ),
]