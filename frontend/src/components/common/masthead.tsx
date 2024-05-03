import React from 'react';
import { Menu, MenuItem } from './menu';
import { Button } from './button';
//import { Route } from 'react-router-dom';
import logo from './../../assets/images/logo.png'; //just a placeholder image

class Header extends React.Component {
  state = {
    isLoggedIn: false // You'll typically get this from your app's state
  };

  handleLogin = () => {
    window.location.href = '/login';
    // Implement login logic
    //this.setState({ isLoggedIn: true });
  };

  handleLogout = () => {
    // Implement logout logic
    this.setState({ isLoggedIn: false });
  };

  render() {
    const { isLoggedIn } = this.state;

    return (
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '10px', borderBottom:'2px solid var(--dark-brown)', marginBottom: '20px', backgroundColor: "var(--secondary-green)" }}>
        {/* Logo */}
        <div>
          <img src={logo} alt="Logo" style={{ height: '50px' }} />
        </div>

        {/* Name */}
        <div>
          <h1>PlantCatalog</h1>
        </div>

        {/* Menu */}
        <div>
          {isLoggedIn ? (
            <Menu>
              <MenuItem id="logout" onClick={this.handleLogout}>Log Out</MenuItem>
              <MenuItem id="myInfo">My Info</MenuItem>
            </Menu>
          ) : (
            <Button onClick={this.handleLogin}>Log In</Button>
          )}
        </div>
      </div>
    );
  }
}

export default Header;
