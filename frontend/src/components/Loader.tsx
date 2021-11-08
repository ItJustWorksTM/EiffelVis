import React from 'react'
import { Spinner } from 'react-bootstrap'
import styles from '../css/spinner.module.css'

const Loader: React.FC = () => (
  <Spinner
    animation="border"
    variant="info"
    role="status"
    className={styles.spinner}
  >
    <span className="visually-hidden">Loading...</span>
  </Spinner>
)

export default Loader
