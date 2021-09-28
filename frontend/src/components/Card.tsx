import React from 'react'
import styles from '../css/card.module.css'

interface CardProps {
  id: String
}

const Card: React.FC<CardProps> = ({ id }) => (
  <div className={styles.cardContainer}>
    <div className={styles.cardHeading}>
      <h5>Event Information</h5>
    </div>
    <div className={styles.cardBody}>
      <ul>
        <li>
          <div className={styles.row}>
            <div>Event ID: </div>
            <div>{id}</div>
          </div>
        </li>
      </ul>
    </div>
  </div>
)

export default Card
