var searchIndex = JSON.parse('{\
"eiffelvis":{"doc":"Combines all individual eiffelvis_* libraries into the …","t":[3,12,11,11,12,11,11,11,11,11,5,12,12,12,12,12,11,11,11,11],"n":["Cli","address","borrow","borrow_mut","chunk_size","clap","fmt","from","from_clap","into","main","max_chunks","port","rmq_queue","rmq_uri","timeout","try_from","try_into","type_id","vzip"],"q":["eiffelvis","","","","","","","","","","","","","","","","","","",""],"d":["Command line options","HTTP host address","","","Maximum amount of events a single chunk will hold","","","","","","Starts all the services that make up EiffelVis.","Maximum amount of chunks stored in memory","HTTP host port","AMQP queue","AMQP URI","AMQP reconnect timeout","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1],"f":[null,null,[[]],[[]],null,[[],["app",3]],[[["formatter",3]],["result",6]],[[]],[[["argmatches",3]]],[[]],[[]],null,null,null,null,null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]]],"p":[[3,"Cli"]]},\
"eiffelvis_core":{"doc":"The <em>core</em> logic library for eiffelvis.","t":[0,0,0,0,0,0,5,0,0,0,8,8,10,10,10,10,3,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,13,13,4,4,3,13,13,13,3,4,13,13,13,13,3,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,16,16,16,16,16,16,16,8,8,8,8,8,16,16,16,8,8,16,16,8,8,8,16,8,6,16,6,16,6,16,10,10,11,10,10,10,10,10,11,10,10,10,10,0,3,3,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,8,3,11,11,11,11,11,11,11,11,11,11,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["algorithms","domain","graph","graph_storage","query","tracked_query","depth_first","app","types","user_queries","EiffelGraph","EiffelVisApp","dump","get_event","get_subgraph_with_roots","push","BaseData","BaseEvent","BaseLink","BaseMeta","LeanEvent","MetaSource","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","data","default","default","default","default","default","default","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","edges","eq","equivalent","event_type","event_type","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","host","id","id","into","into","into","into","into","into","link_type","links","meta","name","ne","partial_cmp","serialize","serialize","serialize","serialize","serialize","serialize","source","tags","target","time","time","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","version","Absolute","AsRoots","Collection","EventFilter","EventFilterMeta","Forward","Id","Ids","Query","RangeFilterBound","SourceHost","SourceName","Tag","Time","TrackedQuery","Type","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","deserialize","deserialize","deserialize","deserialize","deserialize","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","handle","into","into","into","into","into","into","new","serialize","serialize","serialize","serialize","serialize","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","host","id","name","name","tag","val","val","val","Data","Data","EdgeData","EdgeData","EdgeData","EdgeItem","EdgeIterator","Graph","HasNode","HasNodeIter","HasNodeRangeIter","Idx","Idx","Idx","Idx","Indexable","Item","Item","Item","ItemEdge","ItemIter","Key","Key","Meta","NodeIterType","NodeIterType","NodeRangeIterType","NodeRangeIterType","NodeType","NodeType","add_edge","add_node","add_node_with_edges","data","data","edges","get","id","index","items","node_count","range","target","chunked_storage","ChunkedGraph","ChunkedIndex","EdgeData","EdgeIter","Element","NodeData","NodeIter","add_edge","add_node","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","chunks","clone","clone_into","cmp","data","data","dump","eq","equivalent","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from_index","gen","get","get","get","get_event","get_hash","get_subgraph_with_roots","hash","idx","into","into","into","into","into","into","into","into_iter","into_iter","items","last","ne","new","new","next","next","node_count","node_count","partial_cmp","push","range","target","to_index","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","GraphQuery","SubGraphs","borrow","borrow_mut","from","into","into_iter","next","roots_for_graph","try_from","try_into","type_id","OwnedBounds","TrackedNodes","TrackedNodesIter","TrackedSubGraphs","TrackedSubGraphsIter","add_id","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","default","end_bound","from","from","from","from","from","handle","handle","ids","into","into","into","into","into","into_iter","into_iter","new","new","next","next","range","start_bound","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id"],"q":["eiffelvis_core","","","","","","eiffelvis_core::algorithms","eiffelvis_core::domain","","","eiffelvis_core::domain::app","","","","","","eiffelvis_core::domain::types","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::domain::user_queries","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::domain::user_queries::EventFilter","","","","","eiffelvis_core::domain::user_queries::RangeFilterBound","","","eiffelvis_core::graph","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::graph_storage","eiffelvis_core::graph_storage::chunked_storage","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::query","","","","","","","","","","","","eiffelvis_core::tracked_query","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Graph traversal algorithms","eiffelvis domains specicifc api","Graph traits","DAG storage implementations","Generic graph query functionality","Generic tracked graph query functionality","A recursive depth first traversal implementation, if user …","Contains [EiffelGraph] and implements some special …","Eiffel Event types","Query types and functions to acquire specific data from an …","","","Returns all current stored events","Looks up the event of given id","Collects sub-graph(’s) for given root nodes","Inserts a new eiffel event into storage","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Uses the result of the filtered nodes as roots and …","Used collection method, selected variant is run <strong>after</strong> …","","","Does not do anything","Specific ids","","Describes a query to collect nodes from an eiffel graph, …","","meta.source.host","meta.source.name","meta.tags","","A tracked query, only returns new matches.","Event Type","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Collects new nodes that match the query since the last …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Main graph trait","","","","","","","","","","","","","","","","Meta graph trait, defines the base types of a graph such …","","","","","","","Creates a new edge between a and b with given edge data","Creates a new edge with given data without any edges","Convenience function, implemented in terms of …","Returns the data associated with this node","Returns the data associated with this edge","Returns an iterator over the edges this node has","","Returns the id associated with this node","","","Returns the total amount of nodes this graph holds","","Returns the index of the node this edge targets","A storage type that employs the chunked fixed-size …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Translates given internal index to key","","","","Indexes the graph as if it was a linear storage like Vec","","","","","","","","","","","","","","","","Returns the index of the most recent added node","","","","","","","","","","","","Translates given key to internal index. This is generally …","","","","","","","","","","","","","","","","","","","","","","","","An iterator that takes graph nodes and yields the subgraph …","","","","","","Yields the next node, guarantees nodes are yielded in …","Consumes the iterator, used up nodes will be treated as …","","","","Pair of RangeBounds implementation that owns it’s bounds","Yields only the nodes that have been added to graph since …","","Tracked version of crate::query::SubGraphs, behaves in the …","","Note only events that are newer than the current state are …","","","","","","","","","","","","","","","","","","Returns an iterator over the newly added nodes since the …","Returns an iterator over the newly added nodes since the …","","","","","","","","","Creates a new unbounded instance","","","","Creates a new bounded instance, works like ItemIter::range …","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,2,3,4,5,6,7,2,3,4,5,6,7,2,3,4,5,6,7,2,3,4,5,6,7,5,6,2,3,4,5,6,7,2,3,4,5,6,7,7,5,5,3,7,2,3,4,5,6,7,2,3,4,5,6,7,7,4,3,7,2,3,4,5,6,7,5,6,6,4,5,5,2,3,4,5,6,7,3,3,5,3,7,2,3,4,5,6,7,2,3,4,5,6,7,2,3,4,5,6,7,2,3,4,5,6,7,3,8,9,0,0,0,9,10,8,0,0,10,10,10,8,0,10,11,12,8,13,10,9,11,12,8,13,10,9,12,8,13,10,9,12,8,13,10,9,12,8,13,10,9,12,8,13,10,9,11,12,8,13,10,9,11,11,12,8,13,10,9,11,12,8,13,10,9,12,8,13,10,9,11,12,8,13,10,9,11,12,8,13,10,9,11,12,8,13,10,9,14,15,16,17,18,19,20,21,22,23,22,23,24,23,23,0,0,0,0,0,22,23,24,0,0,25,26,0,0,0,22,0,0,25,0,26,0,27,28,28,28,23,24,23,29,23,29,30,28,30,24,0,0,0,0,0,0,0,0,31,31,32,33,34,31,35,36,37,32,33,34,31,35,36,37,31,34,34,34,36,37,31,34,34,34,31,35,36,37,32,33,34,31,35,36,37,31,34,31,31,31,31,34,31,34,34,32,33,34,31,35,36,37,32,33,31,31,34,34,31,32,33,31,31,34,31,31,37,31,34,32,33,34,31,35,36,37,32,33,34,31,35,36,37,32,33,34,31,35,36,37,0,0,38,38,38,38,38,38,39,38,38,38,0,0,0,0,0,40,41,42,40,43,44,41,42,40,43,44,41,44,41,42,40,43,44,41,40,40,41,42,40,43,44,42,43,41,40,42,43,41,44,41,42,40,43,44,41,42,40,43,44,41,42,40,43,44],"f":[null,null,null,null,null,null,[[]],null,null,null,null,null,[[],[["from",8,["baseevent"]],["baseevent",3],["vec",3]]],[[["uuid",3]],[["baseevent",3],["option",4,["baseevent"]]]],[[],[["from",8,["baseevent"]],["baseevent",3],["vec",3]]],[[["baseevent",3]]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["basedata",3]],[[],["basemeta",3]],[[],["metasource",3]],[[],["baselink",3]],[[],["baseevent",3]],[[],["leanevent",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[["baselink",3]],["ordering",4]],null,[[],["basedata",3]],[[],["basemeta",3]],[[],["metasource",3]],[[],["baselink",3]],[[],["baseevent",3]],[[],["leanevent",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],null,[[["baselink",3]],["bool",15]],[[],["bool",15]],null,null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[["baseevent",3]]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,[[["baselink",3]],["bool",15]],[[["baselink",3]],[["option",4,["ordering"]],["ordering",4]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["query",3]],[[],["rangefilterbound",4]],[[],["eventfiltermeta",3]],[[],["eventfilter",4]],[[],["collection",4]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["vec",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[["query",3]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[]],[[],[["option",4,["nodetype"]],["nodetype",6]]],[[]],[[],["nodetype",6]],[[],["nodeitertype",6]],[[],["usize",15]],[[],["noderangeitertype",6]],[[]],null,null,null,null,null,null,null,null,[[]],[[],[["option",4,["chunkedindex"]],["chunkedindex",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["usize",15]],[[],["chunkedindex",3]],[[]],[[["chunkedindex",3]],["ordering",4]],null,null,[[],[["global",3],["vec",3,["global"]]]],[[["chunkedindex",3]],["bool",15]],[[],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["chunkedindex",3]],["option",4]],[[],["u32",15]],[[["chunkedindex",3]],[["nodetype",6],["option",4,["nodetype"]]]],[[],[["nodetype",6],["option",4,["nodetype"]]]],[[["usize",15]],[["nodetype",6],["option",4,["nodetype"]]]],[[["uuid",3]],[["baseevent",3],["option",4,["baseevent"]]]],[[],["u64",15]],[[],[["global",3],["vec",3,["global"]]]],[[]],[[],["u32",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["nodeitertype",6]],[[],[["option",4,["chunkedindex"]],["chunkedindex",3]]],[[["chunkedindex",3]],["bool",15]],[[["u32",15]],["chunkedindex",3]],[[["usize",15],["u32",15]]],[[],["option",4]],[[],["option",4]],[[],["usize",15]],[[],["usize",15]],[[["chunkedindex",3]],[["option",4,["ordering"]],["ordering",4]]],[[["baseevent",3]]],[[],["noderangeitertype",6]],null,[[],[["option",4,["chunkedindex"]],["chunkedindex",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,[[]],[[]],[[]],[[]],[[]],[[],["option",4]],[[],["subgraphs",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["bound",4]],[[]],[[]],[[]],[[]],[[]],[[],["trackednodesiter",3]],[[],["trackedsubgraphsiter",3]],[[],["vec",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["vec",3]]],[[],["option",4]],[[],["option",4]],[[]],[[],["bound",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[8,"EiffelVisApp"],[3,"BaseData"],[3,"BaseMeta"],[3,"MetaSource"],[3,"BaseLink"],[3,"BaseEvent"],[3,"LeanEvent"],[4,"RangeFilterBound"],[4,"Collection"],[4,"EventFilter"],[3,"TrackedQuery"],[3,"Query"],[3,"EventFilterMeta"],[13,"SourceHost"],[13,"Id"],[13,"Type"],[13,"SourceName"],[13,"Tag"],[13,"Absolute"],[13,"Time"],[13,"Ids"],[8,"Meta"],[8,"Item"],[8,"ItemEdge"],[8,"HasNodeIter"],[8,"HasNodeRangeIter"],[8,"HasNode"],[8,"Graph"],[8,"Indexable"],[8,"ItemIter"],[3,"ChunkedGraph"],[3,"NodeIter"],[3,"EdgeIter"],[3,"ChunkedIndex"],[3,"Element"],[3,"NodeData"],[3,"EdgeData"],[3,"SubGraphs"],[8,"GraphQuery"],[3,"TrackedSubGraphs"],[3,"TrackedNodes"],[3,"TrackedNodesIter"],[3,"TrackedSubGraphsIter"],[3,"OwnedBounds"]]},\
"eiffelvis_gen":{"doc":"A library for generating mock eiffel events that actually …","t":[0,0,3,3,3,3,3,3,6,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["event_set","generator","Event","EventBorrow","EventSet","EventSetBuilder","Link","LinkBorrow","LinkTargets","add_event","add_event_set","add_link","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","build","build","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","default","default","default","default","events","fmt","fmt","fmt","from","from","from","from","from","from","from","get_event","get_link","into","into","into","into","into","into","link","link_count","links","multiple_allowed","name","name","new","new","new","targets","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","version","vzip","vzip","vzip","vzip","vzip","vzip","with_link","with_req_link","with_target","EventGenerator","Iter","borrow","borrow","borrow_mut","borrow_mut","default","from","from","into","into","into_iter","iter","new","next","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip"],"q":["eiffelvis_gen","","eiffelvis_gen::event_set","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_gen::generator","","","","","","","","","","","","","","","","","","","","","",""],"d":["Types that are used to describe Events.","Generator that stamps out events.","Describes and Eiffel Event. Note: as of yet, the <code>data</code> …","Proxy type that represents a borrowed Event obtained from …","EventSet allows you to describe the types of eiffel events …","Provides a way to construct the otherwise non …","Describes an Event link","Proxy type that represents a borrowed Link obtained from …","Represents the 2 states of what a Link can target, either …","Adds an Event to the event set. Note: Event names are …","Adds an existing event set, this is useful when want to …","Adds a Link to the event set. Note: Link names are unique, …","","","","","","","","","","","","","Gives quick access to EventSetBuilder.","Consumes the builder and returns a EventSet. Fails if an …","","","","","","","","","","","","","Provides an iterator over the events present in this set.","","","","","","","","","","","Returns the Event that matches the given name.","Returns the Link that matches the given name.","","","","","","","","","Returns an iterator over the links of this event.","","","","Creates a new Link with given name. <code>allow_many</code> hints to …","Creates a new Event with given name and version, see …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Builder function that adds a non required link by name to …","Builder function that adds a <strong>required</strong> link by name to self.","Builder function that adds a target to self.","Holds on to all the data needed to generate a infinite …","","","","","","","","","","","","Creates a new <strong>infinite</strong> event iterator. Every Iter made …","Constructs a new EventGenerator,","Yields a new event, fails if no events could be created …","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,1,1,1,2,3,4,1,5,6,2,3,4,1,5,6,2,1,3,4,5,6,3,4,5,6,2,3,4,1,2,2,3,4,2,3,3,4,1,5,6,2,2,2,3,4,1,5,6,5,5,5,6,5,6,3,4,1,6,3,4,5,6,2,3,4,1,5,6,2,3,4,1,5,6,2,3,4,1,5,6,5,2,3,4,1,5,6,4,4,3,0,0,7,8,7,8,7,7,8,7,8,8,7,7,8,7,8,7,8,7,8,7,8],"f":[null,null,null,null,null,null,null,null,null,[[["event",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["eventsetbuilder",3]],[[],[["eventset",3],["option",4,["eventset"]]]],[[],["link",3]],[[],["event",3]],[[],["eventborrow",3]],[[],["linkborrow",3]],[[]],[[]],[[]],[[]],[[],["eventset",3]],[[]],[[],["event",3]],[[],["eventsetbuilder",3]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],[["option",4,["eventborrow"]],["eventborrow",3]]],[[["str",15]],[["linkborrow",3],["option",4,["linkborrow"]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],[["linkborrow",3],["option",4,["linkborrow"]]]],[[],["usize",15]],[[]],[[],["bool",15]],[[],["str",15]],[[],["str",15]],[[["bool",15]]],[[]],[[]],[[],["option",4]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["str",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["iter",3]],[[["usize",15],["eventset",3]]],[[],["option",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]]],"p":[[3,"EventSetBuilder"],[3,"EventSet"],[3,"Link"],[3,"Event"],[3,"EventBorrow"],[3,"LinkBorrow"],[3,"EventGenerator"],[3,"Iter"]]},\
"eiffelvis_http":{"doc":"An HTTP frontend for eiffelvis_core","t":[8,5],"n":["EiffelVisHttpApp","app"],"q":["eiffelvis_http",""],"d":["","Takes an eiffelvis app and binds the http server on the …"],"i":[0,0],"f":[null,[[["rwlock",3],["arc",3,["rwlock"]],["string",3],["u16",15]]]],"p":[]},\
"eiffelvis_stream":{"doc":"A simple library that hides away all the details and gives …","t":[0,0,3,11,11,11,11,11,11,11,11,11,11],"n":["ampq","gen","AmpqStream","borrow","borrow_mut","from","into","new","next","try_from","try_into","type_id","vzip"],"q":["eiffelvis_stream","","eiffelvis_stream::ampq","","","","","","","","","",""],"d":["Provides an event stream from an ampq connection","Provides an event stream produced by eiffelvis_gen","","","","","","","Yields the next message. Returns None on failure, …","","","",""],"i":[0,0,0,1,1,1,1,1,1,1,1,1,1],"f":[null,null,null,[[]],[[]],[[]],[[]],[[["cow",4,["str"]],["str",15]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]]],"p":[[3,"AmpqStream"]]},\
"event_sender":{"doc":"","t":[3,5,11,11,12,11,12,12,11,11,11,12,5,12,12,12,11,11,11,12,11],"n":["Cli","app","borrow","borrow_mut","burst","clap","count","exchange","from","from_clap","into","latency","main","replay","routing_key","seed","try_from","try_into","type_id","url","vzip"],"q":["event_sender","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","Amount of events to send before introducing another delay …","","Total amount of events to be sent (note: multiplied with …","Ampq exchange to send events to","","","","Time in milliseconds to sleep before emitting a new burst …","","","Routing key used for ampq connections","Random seed used to create event data","","","","URL to amqp server",""],"i":[0,0,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1],"f":[null,[[]],[[]],[[]],null,[[],["app",3]],null,null,[[]],[[["argmatches",3]]],[[]],null,[[],["result",6]],null,null,null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]]],"p":[[3,"Cli"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};