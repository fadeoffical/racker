FROM ubuntu:latest

WORKDIR "/var/racker"

COPY target/release/racker /bin/racker

CMD ["racker", "--host", "0.0.0.0"]
