import React from 'react';
import { Menu, MenuItem } from './menu';
import { Button } from './button';
import { Link } from 'react-router-dom';
import logo from './../../assets/images/logo.png'; //just a placeholder image
import { useAuth } from '../context/AuthContex';

const Header = () => {
  const { user, logout } = useAuth();


  const handleLogout = () => {
    logout();
  }

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
        {user ? (
          <Menu>
            <MenuItem id="logout" onClick={handleLogout}>Log Out</MenuItem>
            <MenuItem id="myInfo">My Info</MenuItem>
          </Menu>
        ) : (
          <Link to="/Login"> <Button>Log In</Button></Link>
        )}
      </div>
    </div>
  );
}

export default Header;
