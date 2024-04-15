# User Management API

This is a simple HTTP API server built using Actix Web for managing user registration, login, and retrieval of user data. It utilizes Diesel for database interaction and runs a PostgreSQL database within a Docker container.

## Installation

1. Clone this repository:

git clone https://github.com/BillArmsty/rusty-exercise.git
cd rusty-exercise


2. Ensure you have Docker installed on your system. You can download and install Docker from [Docker's official website](https://www.docker.com/get-started).

3. Build and run the Docker container:

docker-compose up --build


4. The server will be accessible at `http://localhost:8080`.

## Endpoints

### Register new user

### POST /register

Request Body:

{
  "email": "user@example.com",
  "name": "John Doe",
  "password": "securepassword"
}

### Log in user
### POST /login

Request Body:

{
  "email": "user@example.com",
  "password": "securepassword"
}

Response:

200 OK with a COOKIE if login is successful.
401 Unauthorized if the credentials are invalid.

### Get list of all registered users
### GET /users

Response:

200 OK with a JSON array of all registered users.
401 Unauthorized if the request is not authenticated.



This readme should give a clear overview of the project, how to set it up, and how to use the API endpoints while adhering to the given requirements. Let me know if you need further adjustments or details!
