import React from 'react'
import { Image } from 'react-bootstrap'
import { LinkContainer } from 'react-router-bootstrap'
import { Bezier, Github, InfoCircle } from 'react-bootstrap-icons'
import styles from '../css/sideNav.module.css'

import logo from '../images/logo.png'

const SideNav: React.FC = () => (
  <div className={styles.navBar}>
    <div className={styles.logoContainer}>
      <LinkContainer to="/">
        <Image src={logo} alt="logo" width="55" />
      </LinkContainer>
    </div>
    <ul className={styles.MenuItems}>
      <LinkContainer to="/about">
        <li className={styles.MenuItem}>
          <InfoCircle size={25} />
        </li>
      </LinkContainer>
      <LinkContainer to="/">
        <li className={styles.MenuItem}>
          <Bezier size={25} />
        </li>
      </LinkContainer>
    </ul>
    <footer className={styles.navFooter}>
      <a
        target="_blank"
        href="https://github.com/ItJustWorksTM"
        rel="noreferrer"
      >
        <Github size={25} />
      </a>
    </footer>
  </div>
)

export default SideNav
