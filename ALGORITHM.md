# Algorithm behind EiffelVis

## Concept

Data visualization is a part of the data science field. Our original idea for the visualization of Eiffel events was based on previous research performed within this field. Conceptually, Eiffel events are always linked back to events which have occured before. An event cannot be linked to a future event which has never ocurred. Fundamentally, this means the structure of Eiffel events is similar to that of a [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph) or DAG.

DAG's can be visualized in multiple forms, however, we will focus on two types; topological ordering and hierarchical structure.
