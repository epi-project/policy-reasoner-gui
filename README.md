# Policy Reasoner GUI

This repository hosts a visual frontend for the [Policy Reasoner project](https://github.com/epi-project/policy-reasoner).

The project comprises two main components: a React web application and a server component written in Rust. The web application is hosted by the server component, which primarily acts as a proxy between the frontend and the underlying policy reasoner.

This application serves as a demonstration platform for showcasing the workings of the policy reasoner in demo settings. However, it can also function as a management tool for the policy reasoner, albeit currently limited to default implementations of the policy reasoner's modules/interfaces, such as the eflint-json connector and JWT authentication.

## Running the Application 

### Docker

The most convenient method to run the application is through Docker along with Docker Compose. Ensure you have both Docker and Docker Compose installed by referring to the [official Docker documentation](https://docs.docker.com/engine/install/) and [Docker Compose documentation](https://docs.docker.com/compose/install/).

To build and run the application, navigate to the root folder of the repository and execute the following command:

```bash
$ docker compose up -d
```

If the Policy Reasoner is running on an address different from `http://localhost:3030`, provide the reasoner's address with the `CHECKER_ADDR` environment variable.

```bash
$ CHECKER_ADDR="http://the-reasoners-address:1234" docker compose up -d
```

The application should now be accessible at [http://localhost:3001](http://localhost:3001).

### Manual Setup

#### Building `eflint-to-json`

The application relies on the `eflint-to-json` binary for converting between the eFLINT DSL and the eFLINT JSON Specification formats. This binary is part of the Go `eflint-server` implementation, available [here](https://github.com/Olaf-Erkemeij/eflint-server) (although this project uses a fork [here](https://github.com/epi-project/eflint-server-go)).

To compile the `eflint-to-json` application, follow these steps:

1. Clone the repository (`git clone https://github.com/epi-project/eflint-server-go`).
2. Navigate to the `cmd/eflint-to-json` directory within the cloned repository (`cd eflint-server-go/cmd/eflint-to-json`).
3. Execute the following command:

```bash
$ go build -o path/to/policy-reasoner-client-backend-repo/bin/eflint-to-json .
```

Replace `path/to/policy-reasoner-client-backend-repo` with the appropriate path to your `policy-reasoner-client-backend` directory.

#### Starting the Server

Before starting the server, install [npm](https://www.npmjs.com/en/download) on your machine as a dependency to compile the web interface. Actually building it, though, is taken care of by Cargo.

To start the server, execute the following command:

```bash
$ cargo run
```

If the Policy Reasoner is running on an address different from `http://localhost:3030`, provide the reasoner's address by starting the server with the `--checker-address` option.

```bash
$ cargo run --checker-addres "http://the-reasoners-address:1234"
```

## Using the Application

The application consists of three main screens: the `Reasoner Connector Info`, `Policies`, and `Deliberation API` screens.

### Authentication

Authentication is required before interacting with any of the screens. Upon opening the application, if not authenticated, a login dialog will prompt for a JWT (JSON Web Token). Obtain the JWT using the provided jwt tool from the [Policy Reasoner](https://github.com/epi-project/policy-reasoner) application.

Ensure to use the appropriate JWT for each screen. The `Reasoner Connector Info` and `Policies` screens require a management JWT, while the `Deliberation API` screen needs a deliberation JWT.

It's recommended to acquire a JWT with a long validity to avoid frequent authentication, as there's currently no automatic refreshing mechanism.

Note that The GUI itself does not validate the provided JWT. If the login modal keeps appearing after each action, the provided JWT might be incorrect or expired.

### Reasoner Connector Info Screen

Upon opening the application, you'll land on the `Reasoner Connector Info` screen. Here, you can view the currently installed connector in the policy reasoner and its configuration. Currently, the installed connector is always the eflint-json connector. You can also view the available base definitions and switch between `e-flint` and `eflint-json` formats.

### Policies Screen

On the `Policies` screen, you can view the available policies in the policy reasoner, create new policies, and activate/deactivate existing ones.

### Deliberation API Screen

The `Deliberation API` screen allows you to send requests (`execute task request`, `transfer data request`, and `validate workflow`) to the deliberation API and view the resulting verdicts based on the currently active policy.

### Typical Workflow

Here's a typical workflow for a newly instantiated policy reasoner:

1. **Add a New Policy**

   Open the `Policies` page (login if necessary) and click the `NEW` button to create a new policy. The new policy appears in the left menu as `VERSION NEW`. Note that this version exists only on the frontend and isn't persisted to the policy reasoner until you commit it. The frontend presents a basic policy that you can edit. Once done, enter a commit message and press commit.

   The policy is now persisted as `VERSION 1` (version numbers are automatically assigned) and is shown in the left menu. Once persisted, it becomes read-only, and you can switch between `eflint` and `eflint-json` representations.

   If you create a new policy again, it will copy the currently selected policy as a template.

2. **Activate a Policy**

   Activation is straightforward. Ensure the desired version is selected (lighter gray background) and press the `ACTIVATE` button. An active policy is marked with `(ACTIVE)` after its version number and has a green background when selected.

3. **Interact with the Deliberation API**

   Once a policy is activated, you can interact with the Deliberation API. Write or paste a branescript workflow definition in the Workflow text input (examples can be found in the `tests/branescript` folder). Select the appropriate request and press `EXECUTE`. For some requests (`execute task request`, `transfer data request`), additional information may be required, which can be configured using select boxes automatically filled with appropriate options. The `EXECUTE` button becomes available only when all necessary information is provided.

   View the response in the response area, displaying the verdict or any errors encountered.