# My Docker Application 

This application runs in a Docker container. Follow the instructions below to install Docker and run the application.

![image](https://github.com/omersajid9/first-docker-app/assets/67452047/04a4dfe6-7acf-4c1e-a5cf-6d529a3c7095)

## Prerequisites

- Docker
- Docker Compose

## Installation 

1. Install Docker on your system if it is not already installed:

   - On Linux, follow the [official Docker installation instructions](https://docs.docker.com/engine/install/)
   - On Windows or macOS, install Docker Desktop:  
     - [Docker Desktop for Windows](https://docs.docker.com/docker-for-windows/install/)
     - [Docker Desktop for Mac](https://docs.docker.com/docker-for-mac/install/)

2. Install Docker Compose by following the [official installation instructions](https://docs.docker.com/compose/install/).

3. Clone this repository:
   ```
   git clone https://github.com/omersajid9/first-docker-app.git
    ```
4. Navigate to the repository directory:
   ```
   cd first-docker-app
    ```

## Usage  

To start the application, run:
   ```
   docker compose up
  ```
This will pull the Docker images (if needed), build the containers, and start the application.  

The application will be available at http://localhost:3050.

To stop the application, press Ctrl+C.

## Application Details

This application uses the following:

- A React frontend for the user interface
- A Node/Express API using MySQL to record book reviews
- Another API with PostgreSQL to track tasks
- An Nginx reverse proxy to route request between frontend and backend

Let me know if you have any other questions!
