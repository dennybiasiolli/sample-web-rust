# Building Docker images

- Build the Docker image and run the container

    ```bash
    docker build -t sample-web-rust -t sample-web-rust:bookworm .
    docker run -p 8000:8000 sample-web-rust
    ```

    or the alpine version

    ```bash
    docker build -t sample-web-rust:alpine -f Dockerfile.alpine .
    docker run -p 8000:8000 sample-web-rust:alpine
    ```

- Open a web browser and navigate to http://localhost:8000 to see the "Hello, world!" message.

- Navigate to http://localhost:8000/api to see the JSON response.


## Image size comparison

Bookworm | Alpine
-------- | ------
  106 MB |  16 MB
