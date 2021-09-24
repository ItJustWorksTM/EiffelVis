import React from 'react'
import { Container } from 'react-bootstrap'
import { Route, BrowserRouter as Router } from 'react-router-dom'
import About from './pages/About'
import Home from './pages/Home'
import SideNav from './components/SideNav'
import styles from './css/app.module.css'

const App: React.FC = () => (
  <Router>
    <SideNav />
    <main className={styles.mainContainer}>
      <Container className={styles.routeContainer}>
        <Route path="/about" exact component={About} />
        <Route path="/" exact component={Home} />
      </Container>
    </main>
  </Router>
)

export default App
