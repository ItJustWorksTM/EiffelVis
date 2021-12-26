# Eiffelvis Core

The core implementation of eiffelvis is implemented as a library, aptly named `eiffelvis_core`.

The core library aims to make implementing an actual backend as simple as possible without imposing any decisions on how you consume it.

It tries to provide:

* A generic `Graph` storage trait that describes expectations about how you would commonly store eiffel events and thus is strictly acyclic directed.
* Fast reference storage that implements the aforementioned trait.
* Domain specific types, e.g eiffel event.
* Facilities to query and filter eiffel events in realtime (incrementally).

*read further details in [the api docs](https://itjustworkstm.github.io/EiffelVis/docs/eiffelvis_core/index.html)*