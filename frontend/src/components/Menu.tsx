import React, {useState} from 'react'
import {Button, Card, Col,  Form, Row } from 'react-bootstrap'
import { sendData } from '../interfaces/types';

interface IProps{
  handleSubmit: sendData
}

const Menu: React.FC<IProps>= ({handleSubmit}) => {
    const [collections, setCollections] = useState<string>('');
    const [filters, setFilters] = useState<string>('');
    const [begintime, setBeginTime] = useState<number>(0);
    const [endtime, setEndTime] = useState<number>(-1);
    const [id, setID] = useState<string>('');
    const [type, setType] = useState<string>('')
    const handleCollections = (event: string) => {
        setCollections(event)
    }
    const handleBeginTime = (event: string) => {
        setBeginTime(Number(event))
    }
    const handleFilters = (event: string) => {
        setFilters(event)
    }
    const handleEndTime = (event: string) => {
        setEndTime(Number(event))
    }
    const handleId = (event: string) => {
        setID(event)
    }
        
    const handleType = (event: string) => {
        setType(event)
    }

  return(
    <Card className="bg-dark text-white" style={{ width: '18rem', position:'fixed', right: '0px', top:'0px'}}>   
  <Card.Body>
    <Card.Title>Events</Card.Title>
    <Row>
      <Col><Form.Label>Collections </Form.Label></Col>
    <Col><Form.Select name="collections" id="collections" value={collections} onChange={(e) => handleCollections(e.currentTarget.value as string)} size="sm" aria-label="Select">
        <option value="Forward">Forward</option>
        <option value="AsRoots">AsRoots</option>
        </Form.Select></Col>
    </Row>
      <Row>
        <Col>
        <Form.Label>Filters</Form.Label>
        </Col>
    <Col><Form.Select name="filters" id="filters" value={filters} onChange={(e) => handleFilters(e.currentTarget.value as string)} size="sm" aria-label="Select Filter">
        <option value="None">None</option>
        <option value="Time">Time</option>
        <option value="Ids">Ids</option>
        <option value="Type">Type</option></Form.Select></Col>
      </Row>
    
    <Row>
          <Col><Form.Label>begin</Form.Label></Col>
   <Col> <Form.Control name="begintime" id="begintime" value={begintime} onChange={(e) => handleBeginTime(e.currentTarget.value as string)} size="sm" type="text"/></Col>
    </Row>
    <Row>
      <Col><Form.Label>end</Form.Label></Col>
      <Col><Form.Control name="endtime" id="endtime" value={endtime} onChange={(e) => handleEndTime(e.currentTarget.value as string)} size="sm" type="text"/></Col>
    </Row>
    <Row>
      <Col><Form.Label>node id</Form.Label></Col>
      <Col><Form.Control name="id" id="id" value={id} onChange={(e) => handleId(e.currentTarget.value as string)}  size="sm" type="text"/></Col>
      </Row>
    <Row>
      <Col> <Form.Label>type name</Form.Label> </Col>
    <Col> <Form.Control name="type" id="type" value={type} onChange={(e) => handleType(e.currentTarget.value as string)} size="sm" type="text"/> </Col>
    </Row>
    <div className="d-grid gap-2">
    <Button id="submit" variant="light" onClick={() => handleSubmit(collections, filters, begintime, endtime, id)} size="lg">Submit</Button>{' '}
    </div>

  </Card.Body>
</Card>
  )
};

export default Menu