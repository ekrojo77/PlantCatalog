import React from 'react';
import { useNavigate  } from 'react-router-dom';
import { Button, Center, Container } from '@mantine/core';

const HomePage: React.FC = () => {
  const navigate = useNavigate ();

  const navigateToCreateUser = () => {
    navigate('/create-user');
  };
  return (
    <Center style={{ width: '100vw', height: '100vh'  }}> 
      <Container size="sm" style={{ textAlign: 'center' }}> 
        <h1>Welcome to PlantCatalog</h1>
        <Button onClick={navigateToCreateUser}>Create User</Button>
      </Container>
    </Center>
  );
}

export default HomePage;
