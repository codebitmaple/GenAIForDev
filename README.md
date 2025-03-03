# GenAIForDev

**GenAIForDev** aims to empower developers by incorporating Generative AI into their workflows. This project provides
tools and examples to help you leverage AI in software development.

**Book Link**: 
 - Amazon : https://www.amazon.com/gp/product/B0DYZV9X9N
 - BitMaple - https://bitmaple.com/#book-publishing-details

## Features

- **Code Generation**: Generate code snippets from natural language descriptions.
- **Code Completion**: Enhance coding efficiency with AI-powered code completion.
- **Bug Detection**: Identify and resolve potential bugs through AI analysis.
- **Documentation Assistance**: Create documentation from code and generate code from documentation.

## Installation

# Packaging and Deploying Sage (SkillGenie) backend service

This guide provides step-by-step instructions for packaging and deploying your Rust backend service using Docker. The
application requires an environment file (.env.prod) to run, which contains connection strings for Redis, MongoDB, and
OpenAI keys.

---

## Prerequisites

Before proceeding, ensure you have the following installed on your system:

- **Docker**: [Install Docker](https://docs.docker.com/get-docker/)
- **Rust Toolchain**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Git** (optional, if cloning the repository)

---

## Step 1: Prepare Your Environment File

The application requires a .env.prod file to store sensitive configuration values. Create this file in the root
directory of your project with the following content:

```
# .env.prod

# Redis Connection String
REDIS_URL=redis://<username>:<password>@<host>:<port>/<db>

# MongoDB Connection String
MONGO_URI=mongodb+srv://<username>:<password>@<cluster-url>/<database>?retryWrites=true&w=majority

# OpenAI API Key
OPENAI_API_KEY=<your-openai-api-key>

# .gitignore
.env.prod
```

## Step 2: Write a Dockerfile

```
# Dockerfile

# Stage 1: Build the Rust application

FROM rust:1.73 as builder

# Set the working directory inside the container

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files

COPY Cargo.toml Cargo.lock ./

# Copy the source code

COPY src ./src

# Build the application in release mode

RUN cargo build --release

# Stage 2: Create a minimal runtime image

FROM debian:bullseye-slim

# Install necessary dependencies

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/\*

# Set the working directory

WORKDIR /app

# Copy the compiled binary from the builder stage

COPY --from=builder /app/target/release/<your-app-name> .

# Copy the environment file (ensure users add their own .env.prod)

COPY .env.prod .env

# Expose the port your application listens on

EXPOSE 8080

# Command to run the application

CMD ["./<your-app-name>"]

```

## Step 3: Build the Docker Image

Run the following command to build the Docker image:

```

docker build -t <your-image-name> .

```

Replace <your-image-name> with a name for your Docker image (e.g., rust-backend-service).

## Step 4: Run the Docker Container

Once the image is built, you can run the container using the following command:

```

docker run -d --name <container-name> -p 8080:8080 <your-image-name>

```

Replace <container-name> with a name for your running container. Replace <your-image-name> with the name of the image
you built in Step 3. This command maps port 8080 on your host machine to port 8080 in the container, where your Rust
backend service is running.

## Step 5: Verify the Deployment

To verify that your service is running correctly:

Check the logs of the running container:

```

docker logs <container-name>

```

If everything is set up correctly, your Rust backend service should respond as expected.

Access your service in a web browser or via an HTTP client like curl:

curl http://localhost:8080

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-name`.
3. Make your changes and commit them: `git commit -m 'Add new feature'`.
4. Push to the branch: `git push origin feature-name`.
5. Submit a pull request.

Ensure all tests pass before submitting a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

We thank the open-source community for their contributions and the developers who inspired this project.
