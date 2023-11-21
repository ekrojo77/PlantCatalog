import React, { useState } from 'react';
import { TextInput, Button, Container, Center } from '@mantine/core';

const LoginPage: React.FC = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = async () => {
    try {
        const response = await fetch('/login', {
          method: 'POST', 
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ username, password })
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`); //Improve handling
        }

        //const data = response
    } catch (error) {
        console.error('Error Logging in:', error);
        // Improve handling
      }
  };

  return (
    <Container size="sm" style={{ textAlign: 'center' }}>
      <Center style={{ width: '100vw', height: '100vh'  }}>
        <form onSubmit={handleLogin}>
          <TextInput
            label="Username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
          />
          <TextInput
            label="Password"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
          <Button type="submit" variant="filled">
            Login
          </Button>
        </form>
      </Center>
    </Container>
  );
};

export default LoginPage;
