FROM maven:3-jdk-11

RUN mkdir /jtq
WORKDIR /jtq
COPY . .
RUN mvn clean
RUN mvn install
WORKDIR /jtq/core

CMD ["mvn", "spring-boot:run"]