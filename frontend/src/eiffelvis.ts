import type { Query, Event } from "./apidefinition";

export interface QueryRes {
  repl: string
  error?: string
}

export type ServerMessage = QueryRes | Event[]

// Resumable eifel event iterator received via the shared `EiffelVisConnection`
export class QueryStream {
  private sharedcon: EiffelVisConnection
  private buffer: Event[] = []
  query: Query

  private closed: boolean = true
  private cursor: number = 0

  private tick: any;

  // Constructs a new QueryStream, needs an `EiffelVisConnection` and a `Query`,
  // note: this will not directly activate the stream, see `iter()`
  constructor(sharedcon: EiffelVisConnection, query: Query) {
    this.sharedcon = sharedcon
    this.query = query
    console.log("New query stream with ", query)
  }

  // Calling `.iter()` and awaiting it will take over the connection and submit a query request to the backend
  async iter() {
    await this.activate()

    return this.evgen()
  }

  private async* evgen() {
    while (true) {
      if (this.closed) {
        return
      }
      if (this.cursor < this.buffer.length) {
        yield this.buffer[this.cursor]
        this.cursor += 1
      } else {
        // Wait till tick is called, this means something happened.
        await new Promise((resolve, _) => {
          this.tick = resolve
        })
      }
    }
  }

  // Internal method that establishes the connection to the backend with the stored query
  private async activate() {
    if (!this.closed) {
      // If we still have an active connection we just reset our cursor such that we yield back all the events we have received up till now
      this.cursor = 0
      return
    }
    if (this.buffer.length > 0) {
      // Modify our query to start from the latest event we have in our buffer
      this.query.range_filter = {
        begin: { type: "Ids", val: this.buffer[this.cursor - 1].id },
        end: this.query.range_filter.end
      }
      // Pop the latest event as we will receive it again from the backend
      this.buffer.pop()
    }
    if (await this.sharedcon.request(this)) {
      this.closed = false
      this.cursor = 0
    } else {
      throw Error("Failed to connect")
    }
  }

  // Internal callback handler for when new events are received from the websocket
  onmessage(events: Event[]) {
    this.buffer.push(...events)
    if (this.tick)
      this.tick()
  }

  // Internal callback handler for when `EiffelVisConnection` closes our connection 
  onclose() {
    this.closed = true
    if (this.tick)
      this.tick()
  }

}

// Managed websocket connection which can be shared when adhering to eiffelvis protocol invariants
export class EiffelVisConnection {

  private connection: WebSocket | null;
  private ws_uri: string;
  private http_uri: string;

  // Current QueryStream that is using the connection
  private activestream: QueryStream | null = null;

  // Construct a new `EiffelVisConnection` with a websocket url (ws:// or wss://)
  constructor(backend_url: string, has_ssl: boolean) {
    const backend_proto_ws = has_ssl ? "wss" : "ws";
    const backend_proto_http = has_ssl ? "https" : "http";

    this.ws_uri = `${backend_proto_ws}://${backend_url}/ws`
    this.http_uri = `${backend_proto_http}://${backend_url}`
  }

  private pending: Promise<boolean> | null = null;

  // Internal method used by `QueryStream` to make a new query to the backend.
  async request(querystream: QueryStream): Promise<boolean> {
    // Await the current ongoing request first to prevent sequence break
    if (this.pending)
      await this.pending

    if (this.activestream) {
      this.activestream.onclose()
      this.activestream = null
    }

    // Store promise so that it is known we are actively making a new query
    this.pending = (async (): Promise<boolean> => {
      const data = JSON.stringify(querystream.query)
      const conn = await this.acquire_aconnection()

      if (!conn)
        return false

      // Send query json to backend
      conn.send(data)

      const res: QueryRes = await new Promise((resolve, _) => {
        conn.onmessage = (ev) => {
          const msg: ServerMessage = JSON.parse(ev.data)
          // If the message is an array we know they are left over events from the previous connection,
          // It is fine to skip them as QueryStreams remember where they were at
          if (!Array.isArray(msg)) {
            conn.onmessage = null // Reset onmessage callback as this is an one-shot operation
            resolve(msg as QueryRes)
          }
        }
      })

      console.log(res)

      // Fail if the backend reported failure
      if (res.error) {
        console.log("failed to make query: ", res.error)
        return false
      }

      this.activestream = querystream

      // Delegate any new messages to the QueryStream, they are guarenteed to be events 
      conn.onmessage = (ev) => this.activestream.onmessage(JSON.parse(ev.data))
      this.pending = null
      return true
    })()

    return await this.pending
  }

  // Creates a new websocket and awaits the connection
  private async connect(): Promise<WebSocket | null> {
    const conn = new WebSocket(this.ws_uri)
    return new Promise((resolve, _) => {
      conn.onopen = () => {
        resolve(conn)
      }
      conn.onerror = () => {
        conn.close()
        resolve(null)
      }
    })
  }

  // Returns a websocket connection, by either making a new one or using the existing one
  private async acquire_aconnection(): Promise<WebSocket> {
    while (true) {
      if (!this.connection) {
        this.connection = await this.connect()
        // Wait 1000ms before trying again
        await new Promise((resolve, _) => setTimeout(resolve, 1000))
      }
      else
        break
    }

    // Handle connection closure by reseting the consumer and explicitly closing the websocket
    this.connection.onclose = () => {
      if (this.activestream)
        this.activestream.onclose()
      this.activestream = null
      this.connection.close()
      this.connection = null
    }

    return this.connection
  }

  // Send some data over the current active connection,
  // will not acquire a connection, so will fail if one is not present
  send(data: string | Blob | ArrayBuffer): boolean {
    if (!this.connection)
      return false

    // TODO: Catch errors?
    this.connection.send(data)

    return true
  }

  // TODO: add proper type
  async fetch_node(id: string): Promise<any | null> {
    return await fetch(`${this.http_uri}/get_event/${id}`).then((resp) => resp.json());
  }

}
