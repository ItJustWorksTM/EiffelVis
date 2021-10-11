import React from 'react'
import { Card } from 'react-bootstrap'
import styles from '../css/card.module.css'

interface IProps {
  id: string
  x: number
  y: number
}

const CustomCard: React.FC<IProps> = ({ id, x, y }) => (
  <Card
    className={styles.cardContainer}
    bg="dark"
    text="light"
    style={{ top: `${y}px`, left: `${x}px`, position: 'absolute' }}
  >
    <Card.Header>Event Information</Card.Header>
    <Card.Body>Event ID: {id}</Card.Body>
  </Card>
)

export default CustomCard
