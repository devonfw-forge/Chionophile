from django.urls import include, path

urlpatterns = [
    path(
        'jumpthequeue/services/rest/queuemanagement/v1/queue/', 
        include("queue_management.urls"))
    ,
    path(
        'jumpthequeue/services/rest/visitormanagement/v1/visitor/', 
        include("visitor_management.urls"))
    ,
    path(
        'jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/', 
        include("access_code_management.urls"))
    ,
]
