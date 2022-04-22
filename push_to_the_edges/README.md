# Push to the edges architecture

## Context

One of the objectives of the Chionophile project is to measure how the architecture of an application impacts energy consumption. The initial hypothesis is that by moving all the logic and data possible as close to the users as possible, both the performance and energy consumption will improve. We have implemented this demonstrator following an Edge Computing architecture in order to test this assumption by comparing it with the one following a monolithic server architecture. 

### What is Edge Computing

Edge Computing is a  distributed computing paradigm that aims to move all the data and processing possible as close to the user as it can be, even moving some of this elements to the  Front-End. The main idea is that most of the loading times in modern applications are due to the network, and not the logic itself. It's most common in IoT systems, but companies like Netflix have implemented it because of the multiple advantages that it provides, especially regarding bandwidth consumption.

### Best case usages

In the process of implementing this architecture, we have found that it's greatest strength is working with applications that fall in this two categories:
* Mostly read only applications, where write operations are either infrequent or where consistency is not a priority, such as Netflix or YouTube.
* Data processing applications, where after making the necessary operations, the information is stored and not consumed by clients all over the world. A clear example are IoT systems.

## Differences with the monolithic implementation

The main topologic difference is that now, the application is divided in two types of servers, the edge servers and the central server. Edge servers work as caches close to the client, aiming to reduce the network usage as much as possible.

As previously stated, this architecture shines in the improvement of read operations performance. The use case of the demonstrator does not however fall under any of the two best case usages, because consistency is a priority. That's why there is no improvement in write operations, as they are redirected to the central server (besides not relying in the users network when contacting the central server, using the data center's network of the edge server instead).

All operations that contact the central server store the results in the edge server's database, in order to have as much information as possible to avoid contacting the central server in the majority of reads. To keep the edge server updated and reliable as well as to reduce the amount of unused information, all the data that is not updated for more than 10 minutes is invalidated by a background process.

## Results
