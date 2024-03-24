import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Flex } from '../../../styled-system/jsx';
import { Button } from '../../components/common/button';

import Header from '../../components/common/header';

const HomePage: React.FC = () => {
  const navigate = useNavigate();

  const navigateToCreateUser = () => {
    navigate('/create-user');
  };

  const navigateToLogin = () => {
    navigate('/login');
  };
  return (
    <Flex
      justifyContent="center"
      alignItems="center"
      style={{ width: '100vw', height: '100vh' }}
    >
      <Header /> {/* This is the header component we created earlier */}
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          gap: '10px',
        }}
      >
        <Button onClick={navigateToCreateUser}>Create User</Button>
        <Button onClick={navigateToLogin}>Login</Button>
      </div>
    </Flex>
  );
};

export default HomePage;
