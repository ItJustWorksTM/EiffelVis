import React, { useState } from 'react'
import { Button, Col, Form, Offcanvas } from 'react-bootstrap'
import styles from '../css/offCanvas.module.css'
import { onSubmitDrawer } from '../interfaces/types'
import TimeInput from './TimeInput'

interface IProps {
  show: boolean
  handleClose: Function
  onSubmit: onSubmitDrawer
}
const Drawer: React.FC<IProps> = ({ show, handleClose, onSubmit }) => {
  const [collection, setCollection] = useState('Forward')
  const [filters, setFilters] = useState('None')
  const [beginTime, setBeginTime] = useState('0')
  const [endTime, setEndTime] = useState('-1')
  const [nodeId, setNodeId] = useState('')
  const [typeName, setTypeName] = useState('')

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    onSubmit({ collection, filters, beginTime, endTime, nodeId, typeName })
  }

  return (
    <>
      <Offcanvas
        backdrop={false}
        className={styles.offCanvas}
        placement="end"
        show={show}
        onHide={handleClose}
      >
        <Offcanvas.Header
          className={styles.offCanvasBody}
          closeVariant="white"
          closeButton
        >
          <Offcanvas.Title>Events</Offcanvas.Title>
        </Offcanvas.Header>
        <Offcanvas.Body className={styles.offCanvasBody}>
          <Form onSubmit={handleSubmit}>
            <Form.Group className="mb-2" as={Col} controlId="collections">
              <Form.Label>Collections</Form.Label>
              <Form.Select
                size="sm"
                value={collection}
                onChange={(e: React.ChangeEvent<HTMLSelectElement>) =>
                  setCollection(e.target.value)
                }
              >
                <option value="Forward">Forward</option>
                <option value="AsRoots">AsRoots</option>
              </Form.Select>
            </Form.Group>

            <Form.Group className="mb-2" as={Col} controlId="filters">
              <Form.Label>Filters</Form.Label>
              <Form.Select
                size="sm"
                value={filters}
                onChange={(e: React.ChangeEvent<HTMLSelectElement>) =>
                  setFilters(e.target.value)
                }
              >
                <option value="None">None</option>
                <option value="Time">Time</option>
                <option value="Ids">Ids</option>
                <option value="Types">Types</option>
              </Form.Select>
            </Form.Group>

            <TimeInput
              title="Begin"
              placeHolder="Begin Time"
              handleDateChange={(data: string) => setBeginTime(data)}
            />
            <TimeInput
              title="End"
              placeHolder="End Time"
              handleDateChange={(data: string) => setEndTime(data)}
            />

            <Form.Group className="mb-2" controlId="id">
              <Form.Label>Node Id</Form.Label>
              <Form.Control
                value={nodeId}
                type="text"
                size="sm"
                placeholder="Node Id"
                onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                  setNodeId(e.target.value)
                }
              />
            </Form.Group>

            <Form.Group className="mb-2" controlId="TypeName">
              <Form.Label>Type Name</Form.Label>
              <Form.Control
                value={typeName}
                type="text"
                size="sm"
                placeholder="Type Name"
                onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                  setTypeName(e.target.value)
                }
              />
            </Form.Group>

            <div className="mt-4 d-grid">
              <Button variant="outline-light" type="submit">
                Submit
              </Button>
            </div>
          </Form>
        </Offcanvas.Body>
      </Offcanvas>
    </>
  )
}

export default Drawer
