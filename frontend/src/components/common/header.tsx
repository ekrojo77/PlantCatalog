import React from 'react';
import { Menu, MenuItem } from './menu';
import { Button } from './button';
class Header extends React.Component {
  state = {
    isLoggedIn: false // You'll typically get this from your app's state
  };

  handleLogin = () => {
    // Implement login logic
    this.setState({ isLoggedIn: true });
  };

  handleLogout = () => {
    // Implement logout logic
    this.setState({ isLoggedIn: false });
  };

  render() {
    const { isLoggedIn } = this.state;

    return (
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '10px' }}>
        {/* Logo */}
        <div>
          <img src="path-to-your-logo.png" alt="Logo" style={{ height: '50px' }} />
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
