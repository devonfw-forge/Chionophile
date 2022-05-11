# Results directories:
* **central_monolithic**: Consumption of the Rust backend running in a Nothern Ireland AWS EC2 instance
* **push_to_edges**: Results of the consumption of the Edge Computing version (EC2 instance in Ireland and EC2 instance in Virginia) vs the Rust backend running in a Virginia EC2 instance
* ***temp1***: first local results (java, rust and node without optimizations and improvements with race condition problems)
* ***temp2***: java, rust, node and python running locally and optimized
* ***temp3***: node in cluster mode
* ***temp4***: java, rust, node and python running in docker containers (using nginx for node and python)
* ***temp5***: dotnet running locally
