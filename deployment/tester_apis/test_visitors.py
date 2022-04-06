import requests
import json
import unittest

#######################################################
# VisitorTest:
class VisitorTest(unittest.TestCase):
    def setUp(self):
        new_visitor = {
            "username": "mary@mail.com",
            "name": "Mary",
            "phoneNumber": "1234567",
            "password": "12345",
            "acceptedCommercial": True,
            "acceptedTerms": True,
            "userType": False
        }
        self.BASE_URL = 'http://localhost:8081/jumpthequeue/services/rest/'
        self.VISITOR_URL_POST = 'visitormanagement/v1/visitor/'
        self.VISITOR_URL_SEARCH = 'visitormanagement/v1/visitor/search'
        req = requests.post(self.BASE_URL+self.VISITOR_URL_POST, json=new_visitor)
        self.visitor_id = json.loads(req.text)['id']
        self.VISITOR_URL_GET = 'visitormanagement/v1/visitor/1/'
        self.VISITOR_URL_DELETE = 'visitormanagement/v1/visitor/{}/'.format(self.visitor_id)
        
    def test_get_visitor1(self) :
        req = requests.get(self.BASE_URL+self.VISITOR_URL_GET)
        valid_visitor = {
            'modificationCounter': 1, 
            'id': 1, 
            'username': 'mike@mail.com', 
            'name': 'test', 
            'phoneNumber': '123456789', 
            'password': '1', 
            'acceptedCommercial': False, 
            'acceptedTerms': True, 
            'userType': True
        }
        self.assertEqual(json.loads(req.text), valid_visitor)
    
    def test_search_visitor(self) :
        search1 = {
            "pageable" : {
                "pageNumber" : 0, "pageSize": 2, "sort": []
            }}
        valid_search1 = {
            "content": [
                {
                    "modificationCounter": 1, "id": 1,
                    "username": "mike@mail.com",
                    "name": "test",
                    "phoneNumber": "123456789",
                    "password": "1",
                    "acceptedCommercial": False,
                    "acceptedTerms": True,
                    "userType": True
                }, {
                    "modificationCounter": 1, "id": 2,
                    "username": "peter@mail.com",
                    "name": "test",
                    "phoneNumber": "123456789",
                    "password": "1",
                    "acceptedCommercial": True,
                    "acceptedTerms": True,
                    "userType": False
                }
            ],
            "pageable": {
                "pageNumber": 0, "pageSize": 2, "sort": []
            },
            "totalElements": 38
        }
        search2 = {
            "pageable" : {
                "pageNumber" : 1, "pageSize": 2, "sort": []
            }}
        valid_search2 = {
            "content": [
                {
                    "modificationCounter": 1, "id": 3,
                    "username": "pablo@mail.com",
                    "name": "test",
                    "phoneNumber": "123456789",
                    "password": "1",
                    "acceptedCommercial": False,
                    "acceptedTerms": True,
                    "userType": False
                }, {
                    "modificationCounter": 1, "id": 4,
                    "username": "test1@mail.com",
                    "name": "test",
                    "phoneNumber": "123456789",
                    "password": "1",
                    "acceptedCommercial": False,
                    "acceptedTerms": True,
                    "userType": False
                }
            ],
            "pageable": { 
                "pageNumber": 1, "pageSize": 2, "sort": []
            },
            "totalElements": 38
        }
        search3 = {
            "username": "test1@mail.com",
            "pageable" : {
                "pageNumber" : 0, "pageSize": 2, "sort": []
            }
        }
        valid_search3 = {
            "content": [
                {
                    "modificationCounter": 1, "id": 4,
                    "username": "test1@mail.com",
                    "name": "test",
                    "phoneNumber": "123456789",
                    "password": "1",
                    "acceptedCommercial": False,
                    "acceptedTerms": True,
                    "userType": False
                }
            ],
            "pageable": {
                "pageNumber": 0, "pageSize": 2, "sort": []
            },
            "totalElements": 1
        }
        req = requests.post(self.BASE_URL+self.VISITOR_URL_SEARCH, json=search1)
        if 'totalElements' in json.loads(req.text):
            valid_search1['totalElements'] = json.loads(req.text)['totalElements']
        self.assertEqual(json.loads(req.text), valid_search1)
        req = requests.post(self.BASE_URL+self.VISITOR_URL_SEARCH, json=search2)
        if 'totalElements' in json.loads(req.text):
            valid_search2['totalElements'] = json.loads(req.text)['totalElements']
        self.assertEqual(json.loads(req.text), valid_search2)
        req = requests.post(self.BASE_URL+self.VISITOR_URL_SEARCH, json=search3)
        if 'totalElements' in json.loads(req.text):
            valid_search3['totalElements'] = json.loads(req.text)['totalElements']
        self.assertEqual(json.loads(req.text), valid_search3)

    def test_add_visitor(self) :
        new_visitor = {
            "username": "mary@mail.com",
            "name": "Mary",
            "phoneNumber": "1234567",
            "password": "12345",
            "acceptedCommercial": True,
            "acceptedTerms": True,
            "userType": False
        }
        req = requests.post(self.BASE_URL+self.VISITOR_URL_POST, json=new_visitor)

        self.assertEqual(json.loads(req.text)['name'], new_visitor['name'])
        self.assertEqual(len(dict(json.loads(req.text)).keys())-2, len(new_visitor.keys()))

    def test_update_visitor(self) :
        new_visitor = {
            "id": self.visitor_id,
            "username": "mary@mail.com",
            "name": "Mary1",
            "phoneNumber": "1234567",
            "password": "12345",
            "acceptedCommercial": True,
            "acceptedTerms": True,
            "userType": False
        }
        req = requests.post(self.BASE_URL+self.VISITOR_URL_POST, json=new_visitor)
        self.assertEqual(json.loads(req.text)['id'], new_visitor['id'])
        self.assertEqual(json.loads(req.text)['name'], new_visitor['name'])

    def test_delete_visitor(self) :
        print(4)
        requests.delete(self.BASE_URL+self.VISITOR_URL_DELETE)
        with self.assertRaises(json.JSONDecodeError):
            req = requests.get(self.BASE_URL+self.VISITOR_URL_DELETE)
            print(json.loads(req.text))

if __name__ == '__main__':
    unittest.main()