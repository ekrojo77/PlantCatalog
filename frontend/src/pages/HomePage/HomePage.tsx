import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Flex } from '../../../styled-system/jsx';
import { Button } from '../../components/common/button';
import Masthead from '../../components/common/masthead';
import { useAuth } from '../../components/context/AuthContex';

const HomePage: React.FC = () => {
  const navigate = useNavigate();
  const { user } = useAuth(); 

  const navigateToCreateUser = () => {
    navigate('/create-user');
  };

  const navigateToLogin = () => {
    navigate('/login');
  };

  console.log("User Object: ", user)

  return (
    <Flex
      justifyContent="center"
      alignItems="center"
      style={{ width: '100vw', height: '100vh' }}
    >
      <Masthead /> {/* This is the header component we created earlier */}
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          gap: '10px',
        }}
      >
        {!user && <Button onClick={navigateToCreateUser}>Create User</Button>}
        {!user && <Button onClick={navigateToLogin}>Login</Button>}
      </div>
    </Flex>
  );
};

export default HomePage;
