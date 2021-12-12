import type { Query, QueryRes, ServerMessage, Event } from "./apidefinition";

export const asyncIterable = {

};

export class QueryStream {
  private sharedcon: EiffelVisConnection
  private buffer: Event[] = []
  query: Query

  private closed: boolean = true
  private cursor: number = 0

  private tick: any;

  private async* evgen() {
    while (true) {
      if (this.closed) {
        return
      }
      if (this.cursor < this.buffer.length) {
        yield this.buffer[this.cursor]
        this.cursor += 1
      } else {
        await new Promise((resolve, _) => {
          this.tick = resolve
        })
      }
    }
  }

  constructor(sharedcon: EiffelVisConnection, query: Query) {
    this.sharedcon = sharedcon
    this.query = query
  }

  private async activate() {
    if (!this.closed) {
      this.cursor = 0
      return
    }
    if (this.buffer.length > 0) {
      this.query.range_filter = { begin: { type: "Ids", val: this.buffer[this.cursor - 1].id }, end: this.query.range_filter.end }
      this.buffer.pop()
    }
    if (await this.sharedcon.request(this)) {
      this.closed = false
      this.cursor = 0
    } else {
      throw Error("Failed to connect")
    }
  }

  onmessage(events: Event[]) {
    this.buffer.push(...events)
    if (this.tick)
      this.tick()
  }

  onclose() {
    console.log("query stream closed!")
    this.closed = true
    if (this.tick)
      this.tick()
  }

  async iter() {
    await this.activate()

    return this.evgen()
  }

}

export class EiffelVisConnection {

  private connection: WebSocket | null;
  private uri: string;

  private activestream: QueryStream | null = null;

  constructor(uri: string = "ws://localhost:3001/ws") {
    this.uri = uri
  }

  private pending: Promise<boolean> | null = null;

  async request(querystream: QueryStream): Promise<boolean> {
    if (this.pending)
      await this.pending


    if (this.activestream) {
      this.activestream.onclose()
      this.activestream = null
    }

    this.pending = (async (): Promise<boolean> => {
      const data = JSON.stringify(querystream.query)
      const conn = await this.acquire_aconnection()

      if (!conn)
        return false

      conn.send(data)

      const res: QueryRes = await new Promise((resolve, _) => {
        conn.onmessage = (ev) => {
          const msg: ServerMessage = JSON.parse(ev.data)
          if (!Array.isArray(msg)) {
            resolve(msg as QueryRes)
            conn.onmessage = null
          }
        }
      })

      console.log(res)

      if (res.error) {
        console.log("failed to make query: ", res.error)
        return false
      }

      this.activestream = querystream

      conn.onmessage = (ev) => this.activestream.onmessage(JSON.parse(ev.data))

      return true
    })()

    return this.pending
  }



  private async connect(): Promise<WebSocket | null> {
    const conn = new WebSocket(this.uri)

    return new Promise((resolve, _) => {
      conn.onopen = () => {
        resolve(conn)
      }
      conn.onerror = () => {
        resolve(null)
      }
    })
  }

  private async acquire_aconnection(): Promise<WebSocket> {
    while (true) {
      this.connection = await this.connect()
      if (!this.connection)
        await new Promise((resolve, _) => setTimeout(resolve, 1000))
      else
        break
    }

    this.connection.onclose = () => {
      if (this.activestream)
        this.activestream.onclose()
      this.activestream = null
    }

    return this.connection
  }

  send(data: string | Blob | ArrayBuffer): boolean {
    if (!this.connection)
      return false

    // Catch errors?
    this.connection.send(data)

    return true
  }

}

const testing = async () => {
  const connection = {} as any

  const specific_query = connection.createQuery({})
  const another_query = connection.createQuery({})

    (async () => {
      for await (const event of another_query) {
        // do stuff
      }
    })()

    // some time passes

    (async () => {
      // Awaiting a different query stops the first one
      for await (const event of specific_query) {
        // do stuff
      }
    })()

    // some time passes

    (async () => {
      // awaiting the original one, yields all cached events + new ones
      for await (const event of another_query) {
        // do stuff
      }
    })()

}