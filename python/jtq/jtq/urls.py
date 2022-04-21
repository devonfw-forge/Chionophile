from django.urls import include, path
import environ

env = environ.Env()

urlpatterns = [
    path(
        env('BASE_REST_URL',
             default='jumpthequeue/services/rest')
        + '/queuemanagement/v1/queue/', 
        include("queue_management.urls"))
    ,
    path(
        env('BASE_REST_URL',
             default='jumpthequeue/services/rest')
        + '/visitormanagement/v1/visitor/', 
        include("visitor_management.urls"))
    ,
    path(
        env('BASE_REST_URL',
             default='jumpthequeue/services/rest')
        + '/accesscodemanagement/v1/accesscode/', 
        include("access_code_management.urls"))
    ,
]
