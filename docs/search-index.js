var searchIndex = JSON.parse('{\
"eiffelvis":{"doc":"Combines all individual eiffelvis_* libraries into the …","t":[3,12,11,11,11,11,12,11,11,11,11,11,11,11,5,12,12,12,12,12,12,12,11,11,11,11,11,11],"n":["Cli","address","augment_args","augment_args_for_update","borrow","borrow_mut","chunk_size","fmt","from","from_arg_matches","from_arg_matches_mut","into","into_app","into_app_for_update","main","max_chunks","port","rmq_queue","rmq_uri","timeout","tls_cert","tls_key","try_from","try_into","type_id","update_from_arg_matches","update_from_arg_matches_mut","vzip"],"q":["eiffelvis","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Command line options","HTTP host address","","","","","Maximum amount of events a single chunk will hold","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","Starts all the services that make up EiffelVis.","Maximum amount of chunks stored in memory","HTTP host port","AMQP queue","AMQP URI","AMQP reconnect timeout","Path to TLS certificate pem file","Path to TLS private key pem file","","","","","",""],"i":[0,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2],"f":[0,0,[1,1],[1,1],[[]],[[]],0,[[2,3],4],[[]],[5,[[7,[2,6]]]],[5,[[7,[2,6]]]],[[]],[[],1],[[],1],[[]],0,0,0,0,0,0,0,[[],7],[[],7],[[],8],[[2,5],[[7,[6]]]],[[2,5],[[7,[6]]]],[[]]],"p":[[6,"Command"],[3,"Cli"],[3,"Formatter"],[6,"Result"],[3,"ArgMatches"],[3,"Error"],[4,"Result"],[3,"TypeId"]]},\
"eiffelvis_core":{"doc":"The <em>core</em> logic library for eiffelvis.","t":[0,0,0,0,0,0,5,0,0,0,8,8,10,10,10,10,3,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,13,13,4,4,3,13,13,13,3,4,13,13,13,13,3,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,16,16,16,16,16,16,16,8,8,8,8,8,16,16,16,8,8,16,16,8,8,8,16,8,6,16,6,16,6,16,10,10,11,10,10,10,10,10,11,10,10,10,10,2,0,3,3,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,8,3,11,11,11,11,11,11,11,11,11,11,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["algorithms","domain","graph","graph_storage","query","tracked_query","depth_first","app","types","user_queries","EiffelGraph","EiffelVisApp","dump","get_event","get_subgraph_with_roots","push","BaseData","BaseEvent","BaseLink","BaseMeta","LeanEvent","MetaSource","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","data","default","default","default","default","default","default","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","edges","eq","equivalent","event_type","event_type","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","host","id","id","into","into","into","into","into","into","link_type","links","meta","name","ne","partial_cmp","serialize","serialize","serialize","serialize","serialize","serialize","source","tags","target","time","time","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","version","Absolute","AsRoots","Collection","EventFilter","EventFilterMeta","Forward","Id","Ids","Query","RangeFilterBound","SourceHost","SourceName","Tag","Time","TrackedQuery","Type","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","deserialize","deserialize","deserialize","deserialize","deserialize","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","handle","into","into","into","into","into","into","new","serialize","serialize","serialize","serialize","serialize","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","hosts","ids","names","names","tags","val","val","val","Data","Data","EdgeData","EdgeData","EdgeData","EdgeItem","EdgeIterator","Graph","HasNode","HasNodeIter","HasNodeRangeIter","Idx","Idx","Idx","Idx","Indexable","Item","Item","Item","ItemEdge","ItemIter","Key","Key","Meta","NodeIterType","NodeIterType","NodeRangeIterType","NodeRangeIterType","NodeType","NodeType","add_edge","add_node","add_node_with_edges","data","data","edges","get","id","index","items","node_count","range","target","ChunkedGraph","chunked_storage","ChunkedGraph","ChunkedIndex","EdgeData","EdgeIter","Element","NodeData","NodeIter","add_edge","add_node","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","chunks","clone","clone_into","cmp","data","data","data","dump","eq","equivalent","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from_index","gen","get","get","get","get_event","get_hash","get_subgraph_with_roots","hash","idx","into","into","into","into","into","into","into","into_iter","into_iter","items","last","ne","new","new","next","next","node_count","node_count","partial_cmp","push","range","target","target","to_index","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","GraphQuery","SubGraphs","borrow","borrow_mut","from","into","into_iter","next","roots_for_graph","try_from","try_into","type_id","OwnedBounds","TrackedNodes","TrackedNodesIter","TrackedSubGraphs","TrackedSubGraphsIter","add_id","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","default","end_bound","from","from","from","from","from","handle","handle","ids","into","into","into","into","into","into_iter","into_iter","new","new","next","next","range","start_bound","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id"],"q":["eiffelvis_core","","","","","","eiffelvis_core::algorithms","eiffelvis_core::domain","","","eiffelvis_core::domain::app","","","","","","eiffelvis_core::domain::types","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::domain::user_queries","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::domain::user_queries::EventFilter","","","","","eiffelvis_core::domain::user_queries::RangeFilterBound","","","eiffelvis_core::graph","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::graph_storage","","eiffelvis_core::graph_storage::chunked_storage","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_core::query","","","","","","","","","","","","eiffelvis_core::tracked_query","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Graph traversal algorithms","eiffelvis domains specicifc api","Graph traits","DAG storage implementations","Generic graph query functionality","Generic tracked graph query functionality","A recursive depth first traversal implementation, if user …","Contains [EiffelGraph] and implements some special …","Eiffel Event types","Query types and functions to acquire specific data from an …","","","Returns all current stored events","Looks up the event of given id","Collects sub-graph(’s) for given root nodes","Inserts a new eiffel event into storage","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Uses the result of the filtered nodes as roots and …","Used collection method, selected variant is run <strong>after</strong> …","","","Does not do anything","Specific ids","","Describes a query to collect nodes from an eiffel graph, …","","meta.source.host","meta.source.name","meta.tags","","A tracked query, only returns new matches.","Event Type","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Collects new nodes that match the query since the last …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Main graph trait","","","","","","","","","","","","","","","","Meta graph trait, defines the base types of a graph such …","","","","","","","Creates a new edge between a and b with given edge data","Creates a new edge with given data without any edges","Convenience function, implemented in terms of …","Returns the data associated with this node","Returns the data associated with this edge","Returns an iterator over the edges this node has","","Returns the id associated with this node","","","Returns the total amount of nodes this graph holds","","Returns the index of the node this edge targets","","A storage type that employs the chunked fixed-size …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Translates given internal index to key","","Indexes the graph as if it was a linear storage like Vec","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","Returns the index of the most recent added node","","","","","","","","","","","","","Translates given key to internal index. This is generally …","","","","","","","","","","","","","","","","","","","","","","","","An iterator that takes graph nodes and yields the subgraph …","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Yields the next node, guarantees nodes are yielded in …","Consumes the iterator, used up nodes will be treated as …","","","","Pair of RangeBounds implementation that owns it’s bounds","Yields only the nodes that have been added to graph since …","","Tracked version of crate::query::SubGraphs, behaves in the …","","Note only events that are newer than the current state are …","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns an iterator over the newly added nodes since the …","Returns an iterator over the newly added nodes since the …","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Creates a new unbounded instance","","","","Creates a new bounded instance, works like ItemIter::range …","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,46,46,46,46,0,0,0,0,0,0,6,7,8,9,1,10,6,7,8,9,1,10,6,7,8,9,1,10,6,7,8,9,1,10,9,1,6,7,8,9,1,10,6,7,8,9,1,10,10,9,9,7,10,6,7,8,9,1,10,6,7,8,9,1,10,10,8,7,10,6,7,8,9,1,10,9,1,1,8,9,9,6,7,8,9,1,10,7,7,9,7,10,6,7,8,9,1,10,6,7,8,9,1,10,6,7,8,9,1,10,6,7,8,9,1,10,7,18,21,0,0,0,21,20,18,0,0,20,20,20,18,0,20,22,17,18,19,20,21,22,17,18,19,20,21,17,18,19,20,21,17,18,19,20,21,17,18,19,20,21,17,18,19,20,21,22,17,18,19,20,21,22,22,17,18,19,20,21,22,17,18,19,20,21,17,18,19,20,21,22,17,18,19,20,21,22,17,18,19,20,21,22,17,18,19,20,21,47,48,49,50,51,52,53,54,55,56,55,56,57,56,56,0,0,0,0,0,55,56,57,0,0,58,59,0,0,0,55,0,0,58,0,59,0,60,61,61,61,56,57,56,62,56,62,63,61,63,57,0,0,0,0,0,0,0,0,0,28,28,37,38,29,28,33,34,30,37,38,29,28,33,34,30,28,29,29,29,30,34,30,28,29,29,29,28,33,34,30,37,38,29,28,33,34,30,28,29,28,28,28,28,29,28,29,29,37,38,29,28,33,34,30,37,38,28,28,29,29,28,37,38,28,28,29,28,28,30,30,28,29,37,38,29,28,33,34,30,37,38,29,28,33,34,30,37,38,29,28,33,34,30,0,0,39,39,39,39,39,39,64,39,39,39,0,0,0,0,0,40,41,44,40,45,42,41,44,40,45,42,41,42,41,44,40,45,42,41,40,40,41,44,40,45,42,44,45,41,40,44,45,41,42,41,44,40,45,42,41,44,40,45,42,41,44,40,45,42],"f":[0,0,0,0,0,0,[[]],0,0,0,0,0,[[],[[3,[[2,[1]]]]]],[4,[[5,[1]]]],[[],[[3,[[2,[1]]]]]],[1],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[6,6],[7,7],[8,8],[9,9],[1,1],[10,10],[[]],[[]],[[]],[[]],[[]],[[]],[[9,9],11],0,[[],6],[[],7],[[],8],[[],9],[[],1],[[],10],[[],[[12,[6]]]],[[],[[12,[7]]]],[[],[[12,[8]]]],[[],[[12,[9]]]],[[],[[12,[1]]]],[[],[[12,[10]]]],0,[[9,9],13],[[],13],0,0,[[6,14],15],[[7,14],15],[[8,14],15],[[9,14],15],[[1,14],15],[[10,14],15],[[]],[[]],[[]],[[]],[[]],[[]],[1,10],0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,[[9,9],13],[[9,9],[[5,[11]]]],[6,12],[7,12],[8,12],[9,12],[1,12],[10,12],0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[17,17],[18,18],[19,19],[20,20],[21,21],[[]],[[]],[[]],[[]],[[]],[[],[[12,[17]]]],[[],[[12,[18]]]],[[],[[12,[19]]]],[[],[[12,[20]]]],[[],[[12,[21]]]],[[17,14],15],[[18,14],15],[[19,14],15],[[20,14],15],[[21,14],15],[[]],[[]],[[]],[[]],[[]],[[]],[22,3],[[]],[[]],[[]],[[]],[[]],[[]],[17,22],[17,12],[18,12],[19,12],[20,12],[21,12],[[]],[[]],[[]],[[]],[[]],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[],5],[[],5],[[]],[[]],[[]],[[],[[5,[23]]]],[[]],[[],23],[[],24],[[],25],[[],26],[[]],0,0,0,0,0,0,0,0,0,[[[28,[27]],27,27]],[[[28,[27]],27],[[5,[29]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[28,[27]]],25],[29,29],[[]],[[29,29],11],[30],0,0,[[],[[3,[31]]]],[[29,29],13],[[],13],[[29,14],15],[[[28,[[0,[32,27]],32,32]],14],15],[[[33,[32,32]],14],15],[[[34,[32]],14],15],[[[30,[32]],14],15],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[28,[27]],29],[[5,[27]]]],[29,35],[[[28,[27]],25],[[5,[[23,[[28,[27]]]]]]]],[[[28,[27]],27],[[5,[[23,[[28,[27]]]]]]]],[[[28,[27]],29],[[5,[[23,[[28,[27]]]]]]]],[4,[[5,[1]]]],[[],36],[[],[[3,[31]]]],[29],[29,35],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[28,[27]]],[[24,[[28,[27]]]]]],[[[28,[27]]],[[5,[29]]]],[[29,29],13],[[35,35],29],[[25,35],[[28,[27]]]],[[[37,[27]]],5],[38,5],[[[28,[27]]],25],[[[28,[27]]],25],[[29,29],[[5,[11]]]],[1],[[[28,[27]]],[[26,[[28,[27]]]]]],[30,29],0,[[[28,[27]],27],[[5,[29]]]],[[]],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],0,0,[[]],[[]],[[]],[[]],[[]],[39,5],[[],39],[[],12],[[],12],[[],16],0,0,0,0,0,[40],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],41],[42,43],[[]],[[]],[[]],[[]],[[]],[41,44],[40,45],[40,3],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],41],[3,40],[44,5],[45,5],[[],41],[42,43],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],16],[[],16],[[],16],[[],16],[[],16]],"p":[[3,"BaseEvent"],[8,"From"],[3,"Vec"],[3,"Uuid"],[4,"Option"],[3,"BaseData"],[3,"BaseMeta"],[3,"MetaSource"],[3,"BaseLink"],[3,"LeanEvent"],[4,"Ordering"],[4,"Result"],[15,"bool"],[3,"Formatter"],[6,"Result"],[3,"TypeId"],[3,"Query"],[4,"RangeFilterBound"],[3,"EventFilterMeta"],[4,"EventFilter"],[4,"Collection"],[3,"TrackedQuery"],[6,"NodeType"],[6,"NodeIterType"],[15,"usize"],[6,"NodeRangeIterType"],[8,"Key"],[3,"ChunkedGraph"],[3,"ChunkedIndex"],[3,"EdgeData"],[3,"Global"],[8,"Debug"],[3,"Element"],[3,"NodeData"],[15,"u32"],[15,"u64"],[3,"NodeIter"],[3,"EdgeIter"],[3,"SubGraphs"],[3,"TrackedSubGraphs"],[3,"TrackedNodes"],[3,"OwnedBounds"],[4,"Bound"],[3,"TrackedNodesIter"],[3,"TrackedSubGraphsIter"],[8,"EiffelVisApp"],[13,"SourceHost"],[13,"Id"],[13,"Type"],[13,"SourceName"],[13,"Tag"],[13,"Absolute"],[13,"Time"],[13,"Ids"],[8,"Meta"],[8,"Item"],[8,"ItemEdge"],[8,"HasNodeIter"],[8,"HasNodeRangeIter"],[8,"HasNode"],[8,"Graph"],[8,"Indexable"],[8,"ItemIter"],[8,"GraphQuery"]]},\
"eiffelvis_gen":{"doc":"A library for generating mock eiffel events that actually …","t":[0,0,3,3,3,3,3,3,6,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["event_set","generator","Event","EventBorrow","EventSet","EventSetBuilder","Link","LinkBorrow","LinkTargets","add_event","add_event_set","add_link","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","build","build","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","default","default","default","default","events","fmt","fmt","fmt","from","from","from","from","from","from","from","get_event","get_link","into","into","into","into","into","into","link","link_count","links","multiple_allowed","name","name","new","new","new","targets","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","version","vzip","vzip","vzip","vzip","vzip","vzip","with_link","with_req_link","with_target","EventGenerator","Iter","borrow","borrow","borrow_mut","borrow_mut","default","from","from","into","into","into_iter","iter","new","next","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip"],"q":["eiffelvis_gen","","eiffelvis_gen::event_set","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","eiffelvis_gen::generator","","","","","","","","","","","","","","","","","","","","","",""],"d":["Types that are used to describe Events.","Generator that stamps out events.","Describes and Eiffel Event. Note: as of yet, the <code>data</code> …","Proxy type that represents a borrowed Event obtained from …","EventSet allows you to describe the types of eiffel events …","Provides a way to construct the otherwise non …","Describes an Event link","Proxy type that represents a borrowed Link obtained from …","Represents the 2 states of what a Link can target, either …","Adds an Event to the event set. Note: Event names are …","Adds an existing event set, this is useful when want to …","Adds a Link to the event set. Note: Link names are unique, …","","","","","","","","","","","","","Gives quick access to EventSetBuilder.","Consumes the builder and returns a EventSet. Fails if an …","","","","","","","","","","","","","Provides an iterator over the events present in this set.","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the Event that matches the given name.","Returns the Link that matches the given name.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Returns an iterator over the links of this event.","","","","Creates a new Link with given name. <code>allow_many</code> hints to …","Creates a new Event with given name and version, see …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Builder function that adds a non required link by name to …","Builder function that adds a <strong>required</strong> link by name to self.","Builder function that adds a target to self.","Holds on to all the data needed to generate a infinite …","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Creates a new <strong>infinite</strong> event iterator. Every Iter made …","Constructs a new EventGenerator,","Yields a new event, fails if no events could be created …","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,1,1,1,3,5,2,1,7,8,3,5,2,1,7,8,3,1,5,2,7,8,5,2,7,8,3,5,2,1,3,3,5,2,3,5,5,2,1,7,8,3,3,3,5,2,1,7,8,7,7,7,8,7,8,5,2,1,8,5,2,7,8,3,5,2,1,7,8,3,5,2,1,7,8,3,5,2,1,7,8,7,3,5,2,1,7,8,2,2,5,0,0,18,19,18,19,18,18,19,18,19,19,18,18,19,18,19,18,19,18,19,18,19],"f":[0,0,0,0,0,0,0,0,0,[[1,2],1],[[1,[4,[3]]],1],[[1,[4,[5]]],1],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],1],[1,[[6,[3]]]],[5,5],[2,2],[7,7],[8,8],[[]],[[]],[[]],[[]],[[],3],[[],5],[[],2],[[],1],[3,9],[[3,10],11],[[5,10],11],[[2,10],11],[[]],[[]],[[[4,[12]]],5],[[]],[[]],[[]],[[]],[[3,13],[[6,[7]]]],[[3,13],[[6,[8]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[7,13],[[6,[8]]]],[7,14],[7,9],[8,15],[7,13],[8,13],[[[4,[12]],15],5],[[[4,[12]],[4,[12]]],2],[[],1],[8,[[6,[9]]]],[[]],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[7,13],[[]],[[]],[[]],[[]],[[]],[[]],[[2,[4,[12]]],2],[[2,[4,[12]]],2],[[5,[4,[12]]],5],0,0,[[]],[[]],[[]],[[]],[[],18],[[]],[[]],[[]],[[]],[[]],[18,19],[[20,14,14,3],18],[19,6],[[],16],[[],16],[[],16],[[],16],[[],17],[[],17],[[]],[[]]],"p":[[3,"EventSetBuilder"],[3,"Event"],[3,"EventSet"],[8,"Into"],[3,"Link"],[4,"Option"],[3,"EventBorrow"],[3,"LinkBorrow"],[8,"Iterator"],[3,"Formatter"],[6,"Result"],[3,"String"],[15,"str"],[15,"usize"],[15,"bool"],[4,"Result"],[3,"TypeId"],[3,"EventGenerator"],[3,"Iter"],[8,"Hash"]]},\
"eiffelvis_http":{"doc":"An HTTP frontend for eiffelvis_core","t":[8,3,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["EiffelVisHttpApp","Handle","app","borrow","borrow_mut","clone","clone_into","connection_count","default","fmt","from","graceful_shutdown","into","listening","new","shutdown","to_owned","try_from","try_into","type_id","vzip"],"q":["eiffelvis_http","","","","","","","","","","","","","","","","","","","",""],"d":["","A handle for <code>Server</code>.","Takes an eiffelvis app and binds the http server on the …","","","","","Get the number of connections.","","","Returns the argument unchanged.","Gracefully shutdown the server.","Calls <code>U::from(self)</code>.","Returns local address and port when server starts …","Create a new handle.","Shutdown the server.","","","","",""],"i":[0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6],"f":[0,0,[[[3,[[2,[1]]]],4,5,6,7],8],[[]],[[]],[6,6],[[]],[6,9],[[],6],[[6,10],[[12,[11]]]],[[]],[[6,[7,[13]]]],[[]],[6,8],[[],6],[6],[[]],[[],12],[[],12],[[],14],[[]]],"p":[[8,"EiffelVisHttpApp"],[3,"RwLock"],[3,"Arc"],[4,"IpAddr"],[15,"u16"],[3,"Handle"],[4,"Option"],[8,"Future"],[15,"usize"],[3,"Formatter"],[3,"Error"],[4,"Result"],[3,"Duration"],[3,"TypeId"]]},\
"eiffelvis_stream":{"doc":"A simple library that hides away all the details and gives …","t":[0,0,3,11,11,11,11,11,11,11,11,11],"n":["ampq","gen","AmpqStream","borrow","borrow_mut","from","into","new","next","try_from","try_into","type_id"],"q":["eiffelvis_stream","","eiffelvis_stream::ampq","","","","","","","","",""],"d":["Provides an event stream from an ampq connection","Provides an event stream produced by eiffelvis_gen","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Yields the next message. Returns None on failure, …","","",""],"i":[0,0,0,4,4,4,4,4,4,4,4,4],"f":[0,0,0,[[]],[[]],[[]],[[]],[[[2,[1]],[2,[1]],[2,[1]]],3],[4,3],[[],5],[[],5],[[],6]],"p":[[15,"str"],[4,"Cow"],[8,"Future"],[3,"AmpqStream"],[4,"Result"],[3,"TypeId"]]},\
"event_sender":{"doc":"","t":[3,5,11,11,11,11,12,12,12,11,11,11,11,11,11,12,5,12,12,12,11,11,11,11,11,12,11],"n":["Cli","app","augment_args","augment_args_for_update","borrow","borrow_mut","burst","count","exchange","from","from_arg_matches","from_arg_matches_mut","into","into_app","into_app_for_update","latency","main","replay","routing_key","seed","try_from","try_into","type_id","update_from_arg_matches","update_from_arg_matches_mut","url","vzip"],"q":["event_sender","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","Amount of events to send before introducing another delay …","Total amount of events to be sent (note: multiplied with …","Ampq exchange to send events to","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","Time in milliseconds to sleep before emitting a new burst …","","","Routing key used for ampq connections","Random seed used to create event data","","","","","","URL to amqp server",""],"i":[0,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,0,4,4,4,4,4,4,4,4,4,4],"f":[0,[[],1],[2,2],[2,2],[[]],[[]],0,0,0,[[]],[3,[[6,[4,5]]]],[3,[[6,[4,5]]]],[[]],[[],2],[[],2],0,[[],7],0,0,0,[[],6],[[],6],[[],8],[[4,3],[[6,[5]]]],[[4,3],[[6,[5]]]],0,[[]]],"p":[[8,"Future"],[6,"Command"],[3,"ArgMatches"],[3,"Cli"],[3,"Error"],[4,"Result"],[6,"Result"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
