import React from 'react'
import { Button, Card } from 'react-bootstrap'
import styles from '../css/card.module.css'

interface IProps {
  id: string
  x: number
  y: number
  getNodesWithRoot: any
}

const CustomCard: React.FC<IProps> = ({ id, x, y, getNodesWithRoot }) => (
  <Card
    className={styles.cardContainer}
    bg="dark"
    text="light"
    style={{ top: `${y}px`, left: `${x}px`, position: 'absolute' }}
  >
    <Card.Header>Event Information</Card.Header>
    <Card.Body className="d-grid">
      <p>Event ID: {id}</p>
      <Button onClick={() => getNodesWithRoot(id)} variant="outline-light">
        Nodes With This Root
      </Button>
    </Card.Body>
  </Card>
)

export default CustomCard
