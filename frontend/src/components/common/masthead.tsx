import React from 'react';
import * as Menu from '../ui/menu';
import { Button } from '../ui/button';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { Heading } from '../ui/heading';
import logo from './../../assets/images/logo.png'; 

interface UserMenuProps {
  onLogout: () => void;
}

const UserMenu: React.FC<UserMenuProps> = ({ onLogout }) => {
  const commonWidth = '200px'; 
  const navigate = useNavigate();

  const menuStyle: React.CSSProperties = {
    backgroundColor: 'var(--primary-green)',
    border: '1px solid var(--dark-brown)',
    borderRadius: '10px',
    padding: '10px 0',
    position: 'absolute',
    right: '0',
    top: '100%',
    zIndex: 1,
    width: commonWidth, 
  };

  const itemStyle: React.CSSProperties = {
    padding: '10px 20px',
    cursor: 'pointer',
    color: 'var(--off-white)',
    textAlign: 'center',
  };

  const separatorStyle: React.CSSProperties = {
    height: '1px',
    backgroundColor: 'var(--dark-brown)',
    margin: '5px 0',
  };

  return (
    <Menu.Root>
      <Menu.Trigger aria-label="Open user menu">
        <Button style={{ padding: '10px', fontSize: '16px', backgroundColor: 'var(--primary-green)', color: 'var(--off-white)', width: 'auto' }}>
          Menu
        </Button>
      </Menu.Trigger>
      <Menu.Content style={menuStyle}>
        <Menu.Item 
          id="myInfo" 
          aria-label="Go to My Info" 
          style={itemStyle}
          onClick={() => navigate('/userpage')}
          >
          My Info
        </Menu.Item>
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
    <div style={{
      display: 'flex',
      justifyContent: 'space-between',
      alignItems: 'center',
      padding: '10px 20px',
      borderBottom: '2px solid var(--dark-brown)', 
      backgroundColor: 'var(--secondary-green)',
    }}>
      <div style={{ padding: '10px' }}>
        <img src={logo} alt="Logo" style={{ height: '50px', borderRadius: '50%', border: '2px solid var(--dark-brown)' }} />
      </div>

      <div>
        <Heading size="xl" fontWeight="bold" style={{ color: 'var(--primary-green)' }}>PlantCatalog</Heading>
      </div>

      <div style={{ position: 'relative' }}>
        {user ? (
          <UserMenu onLogout={handleLogout} />
        ) : (
          <Link to="/Login">
            <Button style={{
              padding: '10px 20px',
              fontSize: '16px',
              backgroundColor: 'var(--primary-green)',
              color: '#FFFFFF',
              borderRadius: '8px',
              display: 'flex',
              alignItems: 'center',
            }}>
              Log In
            </Button>
          </Link>
        )}
      </div>
    </div>
  );
};

export default Masthead;

