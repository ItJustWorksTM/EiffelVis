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

## **Graph Options**

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

DAG's can be visualized in multiple forms, however, we will focus on two types; topological ordering and hierarchical structure. In early stages on development, we were considering using a hierarchical structure as it worked well with Eiffel events in theory and the visualization aspect was also good. There were two algorithms which were considered to be best suited to this project, [Sugiyama's Layout](https://en.wikipedia.org/wiki/Layered_graph_drawing) and the [Brandes Köpf Algorithm](https://www.semanticscholar.org/paper/Fast-and-Simple-Horizontal-Coordinate-Assignment-Brandes-K%C3%B6pf/69cb129a8963b21775d6382d15b0b447b01eb1f8).

To learn more about these algorithm's please see the links above as the explanation is quite detailed. Briefly, Sugiyama's Layout uses a system of ranks where each event is assigned a rank based on which event it links to. Because Sugiyama's layout is based on event links, the ranks need to be updated everytime a new event is added as it links to an event in the past. If ranks are updated, part of the algorithm needs to be recalculated, this makes the process recursive and the levels of recursion increase as the number of events increases. The Brandes-Köpf algorithm is similar but instead of a vertical graph, they change the alignment to be horizontal.

### The EiffelVis Layout

As outlined in the concept, using Sugiyama's Algorithm causes recursion to go deeper as the number of events increases. This was a problem for our system as we wanted it to be real-time and the time complexity was too high if we used Sugiyama's Layout or the Brandes-Köpf Algorithm.

Therefore, we created our own layout algorithm instead. As all Eiffel events have a `time` field, we based our algorithm on the time at which an event in generated. As the EiffelVis backend sends a sorted list of events (based on time) to the frontend, the algorithm can simply assign a X and Y value to each event in real-time. So events with the same time stamp have the same X axis and if the time changes, the events are assigned to the next X value. Moreoever, if events have the same time, the Y value has older nodes close to the Y axis, and newer nodes further away. This property is implemented so that if the `offset` parameter of the layout is changed, it still maintains the properties of a DAG by always being linked to events in the past. Finally, the time complexity of our algorithm comes down to O(1) complexity, which makes it extremely fast.
