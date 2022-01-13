# EiffelVis Frontend

## Getting started

```bash
# Clone repository
git clone git@github.com:ItJustWorksTM/EiffelVis.git

# Change into the directory
cd EiffelVis/frontend

# Install dependencies
npm install

# Compiles and hot-reloads for development
npm run dev

# Compiles and minifies for production
npm run build
```

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

## Layout Algorithm

### Concept

Data visualization is a part of the data science field. Our original idea for the visualization of Eiffel events was based on previous research performed within this field. Conceptually, Eiffel events are always linked back to events which have occured before. An event cannot be linked to a future event which has never ocurred. Fundamentally, this means the structure of Eiffel events is similar to that of a [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph) or DAG.

DAG's can be visualized in multiple forms, however, we will focus on two types; topological ordering and hierarchical structure.
