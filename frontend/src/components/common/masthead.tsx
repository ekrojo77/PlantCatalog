import React from 'react';
import * as Menu from './menu';
import { Button } from './button';
import { Link } from 'react-router-dom';
import logo from './../../assets/images/logo.png'; // Just a placeholder image
import { useAuth } from '../context/AuthContex';

interface UserMenuProps {
  onLogout: () => void;
}

const UserMenu: React.FC<UserMenuProps> = ({ onLogout }) => {
  const positionerStyle: React.CSSProperties = {
    minWidth: '100%',
    zIndex: 1, 
    left: '0',
  };
  const menuStyle: React.CSSProperties = {
    backgroundColor: 'var(--primary-green)',
    border: '2px solid var(--dark-brown)',
    borderRadius: '5px',
    padding: '10px',
    position: 'absolute',
    right: '0',
    top: '100%',
    zIndex: 1,
  };
  const itemStyle: React.CSSProperties = {
    padding: '10px',
    cursor: 'pointer',
    color: 'var(--off-white)',
  };
  const separatorStyle : React.CSSProperties = {
    height: '2px',
    backgroundColor: 'var(--dark-brown)',
    margin: '5px 0',
  };
  return (
    <Menu.Root>
      <Menu.Trigger aria-label="Open user menu">
        <Button>Menu</Button>
      </Menu.Trigger>
      <Menu.Positioner style={positionerStyle}>
        <Menu.Content style={menuStyle}>
          <Menu.Item id="myInfo" aria-label="Go to My Info" style={itemStyle}>My Info</Menu.Item>
          <Menu.Separator style={separatorStyle}/>
          <Menu.Item id="logout" aria-label="Log Out" onClick={onLogout} style={itemStyle}>Log Out</Menu.Item>
        </Menu.Content>
      </Menu.Positioner>
    </Menu.Root>
  );
};

const Masthead = () => {
  const { user, logout } = useAuth();

  const handleLogout = () => {
    logout();
  };

  return (
    <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '10px', borderBottom: '2px solid var(--dark-brown)', marginBottom: '20px', backgroundColor: 'var(--secondary-green)' }}>
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
          <UserMenu onLogout={handleLogout} />
        ) : (
          <Link to="/Login">
            <Button>Log In</Button>
          </Link>
        )}
      </div>
    </header>
  );
};

export default Masthead;