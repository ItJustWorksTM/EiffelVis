import React from 'react'
import { Button, Card } from 'react-bootstrap'
import styles from '../css/card.module.css'
import { getNodesWithRootFun } from '../interfaces/types'

interface IProps {
  id: string
  time: number
  x: number
  y: number
  getNodesWithRoot: getNodesWithRootFun
}

const CustomCard: React.FC<IProps> = ({ id, time, x, y, getNodesWithRoot }) => (
  <Card
    className={styles.cardContainer}
    bg="dark"
    text="light"
    style={{ top: `${y}px`, left: `${x}px`, position: 'absolute' }}
  >
    <Card.Header>Event Information</Card.Header>
    <Card.Body className="d-grid">
      <p>Event ID: {id}</p>
      <p>Event Time: {time}</p>
      <Button onClick={() => getNodesWithRoot(id)} variant="outline-light">
        Nodes With This Root
      </Button>
    </Card.Body>
  </Card>
)

export default CustomCard
