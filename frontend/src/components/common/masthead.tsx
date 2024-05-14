import React from 'react';
import * as Menu from './menu';
import { Button } from './button';
import { Link } from 'react-router-dom';
import logo from './../../assets/images/logo.png'; // Just a placeholder image
import { useAuth } from '../context/AuthContext';

interface UserMenuProps {
  onLogout: () => void;
}

const UserMenu: React.FC<UserMenuProps> = ({ onLogout }) => {
  const commonWidth = '150px'; // Set the desired width

  const menuStyle: React.CSSProperties = {
    backgroundColor: 'var(--primary-green)',
    border: '2px solid var(--dark-brown)',
    borderRadius: '5px',
    padding: '10px 0',
    position: 'absolute',
    right: '0',
    top: '100%',
    zIndex: 1,
    width: commonWidth, 
  };

  const itemStyle: React.CSSProperties = {
    padding: '10px',
    cursor: 'pointer',
    color: 'var(--off-white)',
    textAlign: 'center', // Center align text for better visual appearance
  };

  const separatorStyle: React.CSSProperties = {
    height: '2px',
    backgroundColor: 'var(--dark-brown)',
    margin: '5px 0',
  };

  return (
    <Menu.Root>
      <Menu.Trigger aria-label="Open user menu">
        <Button style={{ padding: '10px', fontSize: '16px', backgroundColor: 'var(--dark-green)', color: 'var(--off-white)', width: commonWidth }}>
          Menu
        </Button>
      </Menu.Trigger>
      <Menu.Content style={menuStyle}>
        <Menu.Item id="myInfo" aria-label="Go to My Info" style={itemStyle}>My Info</Menu.Item>
        <Menu.Separator style={separatorStyle} />
        <Menu.Item id="logout" aria-label="Log Out" onClick={onLogout} style={itemStyle}>Log Out</Menu.Item>
      </Menu.Content>
    </Menu.Root>
  );
};

const Masthead = () => {
  const { user, logout } = useAuth();

  const handleLogout = () => {
    logout();
  };

  return (
    <header style={{
      display: 'flex',
      justifyContent: 'space-between',
      alignItems: 'center',
      padding: '10px 20px',
      borderBottom: '2px solid var(--dark-brown)', 
      backgroundColor: 'var(--secondary-green)'
    }}>
      {/* Logo */}
      <div style={{ padding: '10px' }}>
        <img src={logo} alt="Logo" style={{ height: '50px', borderRadius: '50%', border: '2px solid var(--dark-brown)' }} />
      </div>

      {/* Name */}
      <div>
        <h1 style={{ fontSize: '2em', fontWeight: 'bold', color: 'var(--dark-green)' }}>PlantCatalog</h1>
      </div>

      {/* Menu */}
      <div style={{ position: 'relative' }}>
        {user ? (
          <UserMenu onLogout={handleLogout} />
        ) : (
          <Link to="/Login">
            <Button style={{ padding: '10px', fontSize: '16px', backgroundColor: 'var(--dark-green)', color: 'var(--off-white)', width: '150px' }}>Log In</Button>
          </Link>
        )}
      </div>
    </header>
  );
};

export default Masthead;
