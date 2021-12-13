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
    console.log("New query stream with ", query)
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

  constructor(uri: string) {
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
            conn.onmessage = null
            resolve(msg as QueryRes)
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
      this.pending = null
      return true
    })()

    return await this.pending
  }

  private async connect(): Promise<WebSocket | null> {
    const conn = new WebSocket(this.uri)
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

  private async acquire_aconnection(): Promise<WebSocket> {

    while (true) {
      if (!this.connection) {
        this.connection = await this.connect()
        await new Promise((resolve, _) => setTimeout(resolve, 1000))
      }
      else
        break
    }

    this.connection.onclose = () => {
      if (this.activestream)
        this.activestream.onclose()
      this.activestream = null
      this.connection.close()
      this.connection = null
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
