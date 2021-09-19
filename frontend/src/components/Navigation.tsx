import React from 'react'
import { Container, Image, Nav, Navbar } from 'react-bootstrap'
import { LinkContainer } from 'react-router-bootstrap'
import logo from '../images/logo.png'

const navigation: React.FC = () => (
  <Navbar collapseOnSelect bg="dark" variant="dark">
    <Container>
      <LinkContainer to="/">
        <Navbar.Brand>
          <Image src={logo} alt="logo" width="50" />
        </Navbar.Brand>
      </LinkContainer>
      <Navbar.Toggle aria-controls="navbar" />
      <Navbar.Collapse>
        <Nav className="ms-auto">
          <LinkContainer to="/about">
            <Nav.Link>About</Nav.Link>
          </LinkContainer>
        </Nav>
      </Navbar.Collapse>
    </Container>
  </Navbar>
)

export default navigation
