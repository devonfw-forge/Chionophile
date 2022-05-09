# Generic benchmark

This a basic program that uses a configuration file in YAML to create load benchmarks using the Goose Framework.

## Objective

Give the option to make load benchmarks in an efficient language like Rust without the need to know how to write it yourself.

## Usage

The configuration options are the following:

- host
- port
- use_https
- base_path
- users
- goose_log
- user_creation_rate
- run_time
- report_file
- request_log
- request_body
- error_log
- debug_log
- max_request_second
- no_reset_metric

All of them are explained in [this page of the Goose documentation (https://docs.rs/goose/latest/goose/config/struct.GooseConfiguration.html)]. 
The only changes are **user_creation_rate** and **max_request_second** that corresponds with **hatch_rate** and **throttle_requests** respectively.

Under the **benchmarks** section you can define a series of request groups that will fall under that benchmark name. Define as many benchmarks as 
you want with as many requests groups and individual requests.

Take into account that all of the requests in a request group will be executed consecutively, but they will run paralelly with other request groups.

The next options are designed to configure a DELETE request after a POST request:
- should_delete
- body_field_for_delete
- delete_end_slash

If should_delete is true, the program will look for the field indicated in body_field_for_delete in the POST request's response. The delete_end_slash indicates 
if a "/" should be added in the end of the URL. 

For example:
1. A POST request is made with the next URL **/visitormanagement/v1/visitor/**
2. It returns the following response:
``
  {
    "id"¨: 123
    "username": "user@mail.com",
    "name": "pepe",
    "phoneNumber": "+34623456789",
    "password": "paco",
    "acceptedTerms": true,
    "acceptedCommercial": false
  }
  
``
 3. should_delete is true
 4. body_field_for_delete is "id"
 5. The program looks for the "id" field in the previous response
 6. If delete_end_slash is true makes a DELETE request with this URL: **/visitormanagement/v1/visitor/123/**
 7. If delete_end_slash is false makes a DELETE request with this URL: **/visitormanagement/v1/visitor/123**
 
