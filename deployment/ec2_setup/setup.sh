# !/bin/bash

cd ~

curl https://sh.rustup.rs -sSf | sh

sudo yum -y install gcc

git clone https://github.com/devonfw-forge/Chionophile.git

sudo yum -y install postgresql-devel

sudo yum -y install openssl-devel

sudo curl -L "https://github.com/docker/compose/releases/download/v2.5.0/docker-compose-$(uname -s)-$(uname -m)" -o /bin

sudo chmod +x /usr/local/bin/docker-compose

