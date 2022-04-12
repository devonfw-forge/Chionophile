# Chionophile
Demonstrator on how to achieve energy efficiency improvements  in software development

## Requirements
- Windows OS with powershell
- gitbash
- node
- npm
- java
- maven
- python
- pip
- rust
- cargo

## Aim
Compare backends development performance and energy consuption with sample application based on Jump The Queue (https://github.com/devonfw/jump-the-queue).

Currently, analyzed backends are:

- Python (Django REST)
- NodeJS (Nest.js)
- Java (Springboot)
- C# (ASP.NET)
- Rust (Actix4)

Backends are completly exchangeable for all the endpoints used in benchmarks.

## Benchmarks
Two benchmarks are executed for different purposes:
1. **Energy consumption benchmark**: for the next 5 minuts, backends will received 14 request per second. This request rate will guarantee that all backends can send responses without raises 404 or 500 errors.
2. **Performance benchmark**: backends will received as request as possible. This will show how many request can process each benchmark.


## Test flow

The testing flow is composed by the following statements:

- Get idle energy consumption for 1 minute
- For each backend:
  - Start Postgres DB docker.
  - Wait for 30s.
  - Launch backend.
  - Wait for 2 min.
  - Start Benchmarks sequentially
  - When benchmarks are finished, the backend and Postgres container are killed.

## Results

The previous execution of the automated script will generate a report in results folder. Some results are currently place in this folder, both in pdf and html. We encorage you to download html as it has dynamic content and will ease results observation. Here is a taste:

### Benchmark 1
|                               |  Rust      |  Java   |  NodeJS  |
|:-----------------------------:|:----------:|:-------:|:--------:|
| Consumption B1 (mWh)          | **631.5**  | 1346.36 | 835      |
| Consumption %                 | -          | +53%    | 24%      |
| Consumption % (idle excluded) | -          | +63%    | 32%      |

### Benchmark 2
|                               |  Rust      |  Java   |  NodeJS  |
|:-----------------------------:|:----------:|:-------:|:--------:|
| Total request B2              | **784076** | 307610  | 43790    |
| Request / second              | **871**    | 341     | 48       |
| Consumption B2 (mWh)          | 2719.69    | 2604.53 | 2220.11  |
| Performance %                 | -          | -60%    | -94%     |