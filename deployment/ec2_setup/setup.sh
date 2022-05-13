# !/bin/bash

cd ~

curl https://sh.rustup.rs -sSf | sh

sudo yum -y install gcc

git clone https://github.com/devonfw-forge/Chionophile.git

sudo yum -y install postgresql-devel

sudo yum -y install openssl-devel
