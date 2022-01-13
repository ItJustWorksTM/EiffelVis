[![EiffelVis](assets/branding/eiffel_vis_eye.svg)](https://github.com/ItJustWorksTM/EiffelVis)

# EiffelVis

[![Sandbox badge](https://img.shields.io/badge/Stage-Sandbox-yellow)](https://github.com/eiffel-community/community/blob/master/PROJECT_LIFECYCLE.md#stage-sandbox)
[![Backend CI](https://github.com/ItJustWorksTM/EiffelVis/actions/workflows/backend_ci.yml/badge.svg)](https://github.com/ItJustWorksTM/EiffelVis/actions/workflows/backend_ci.yml)
[![Frontend CI](https://github.com/ItJustWorksTM/EiffelVis/actions/workflows/frontend_ci.yml/badge.svg)](https://github.com/ItJustWorksTM/EiffelVis/actions/workflows/frontend_ci.yml)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)

EiffelVis is a scallable Eiffel pipeline traffic visualization stack built on Rust & Svelte. Briefly, the Eiffel protocol is a tool which helps control multiple CI/CD Pipelines within or across multiple projects. To learn more about the Eiffel Protocol, please visit the [Eiffel Community](https://eiffel-community.github.io/).

#### **Why should you use EiffelVis?**

The Eiffel Protocol consists of multiple [Eiffel Events](https://github.com/eiffel-community/eiffel/tree/master/eiffel-vocabulary) which each have a defined purpose and structure. In a typical CI pipeline which uses the Eiffel Protocol, there can be hundereds of events which are generated over a short time period. Visualizing large amounts of data such as thousands of Eiffel Events can be difficult, and thats where EiffelVis stands out. EiffelVis is a data visualization and data manipulation tool for Eiffel Events.

# Getting Started

These instructions are an example of how you may run EiffelVis locally.

## Prerequisites

- [NodeJS](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/tools/install)

## Installation

1. Clone the repository using SSH (or HTTP)
   ```bash
   git clone git@github.com:ItJustWorksTM/EiffelVis.git
   ```
2. After going to the root folder where the repository was cloned, install dependencies for the frontend
   ```bash
   cd EiffelVis/frontend
   npm install
   ```
3. Compile and run the frontend
   ```bash
   npm run dev
   ```
4. The frontend will now be running on port `localhost:8080`. If you go to this address in your browser, you should be able to view the EiffelVis client. As you will see, there are no events and there is no graph. To see events we need to set up the backend through which we will also load the graph data.

5. As the client will be running in the current terminal, open a new terminal to run the backend. Go to the root directory of EiffelVis and run the commands
   ```bash
   cd EiffelVis/backend
   cargo run -- --help
   ```

Now that the application is running in these 5 simple steps, your client should look something like this: [insert image]

# Usage

Additional information that will be useful in helping you utilize EiffelVis to it's maximum potential.

## Client

### **Filters**

Idk man

### **Event Information**

The green blob

### **Graph Options**

[insert annotated image with graph menu]

| Graph Option           | Description                                                                                                                                                                                                               | Default Value | Ideal Value Range |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-----------: | :---------------: |
| Offset                 | By default, all events along the Y-axis of the graph will be in a straight line. This might hide edges as they will be stacked on top of each other. Offset makes the Y-axis more curved and displays edges more clearly. |       0       |      0 - 100      |
| Time Collapse          | Controls how the events are collapsed, based on time. Eiffel events use milliseconds since Unix epoch. Decreasing the value will increase the number of events displayed on every x-axis value.                           |     1000      |     0 - 10000     |
| Y-axis Scaling         | Controls the distance between events on the y-axis in case there are too many events generated at the same timestamp.                                                                                                     |     0.99      |       0 - 1       |
| X-axis Node Separation | Controls the distance between events on the x-axis.                                                                                                                                                                       |      60       |     20 - 1000     |
| Y-axis Node Separation | Controls the distance between events on the y-axis.                                                                                                                                                                       |      60       |     20 - 1000     |
| Hue                    | Changes the color of events on the graph, the higher the hue value the more variation in colors.                                                                                                                          |      360      |      0 - 360      |

### **Graph Legend**

Displays the event types and the color associated with each event type. Changing the hue value will update the Legend.

### **Timebar**

Nice

## Backend

For usage instructions regarding the backend, please see the [backend guide](./backend/README.md).

# Deployment

If you would like to deploy the application please refer to the [deployment guide](./DEPLOYMENT.md) for detailed instructions.

# Demo

A demo of our application is hosted at [insert link].

# About the Repository

The contents of this repository are licensed under the [Apache License 2.0](./LICENSE).

To contribute to this repository, please see the [contribution guidelines](./CONTRIBUTING.md).

If you would like to read more about the algorithm used by EiffelVis, please see the [algorithm concept](./ALGORITHM.md).

## Links:

- [How to contribute](./CONTRIBUTING.md)
- [Issue tracker](https://github.com/ItJustWorksTM/EiffelVis/issues)
- [Website](https://itjustworkstm.github.io/EiffelVis/)
- [EiffelVis Backend](./backend)
- [EiffelVis Frontend](./frontend)
