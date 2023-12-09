import React, { useState } from 'react';
import { TextInput, Button, Container, Center } from '@mantine/core';
import { PasswordStrength } from "../../components/common/PasswordStrength"

const CreateUserPage: React.FC = () => {
  const [name, setName] = useState<string>('');
  const [username, setUsername] = useState<string>('');
  const [email, setEmail] = useState<string>('');

  const handleCreateUser = async () => {
    try {
        const getPassword = () => document.querySelector<HTMLInputElement>('#password')?.value;
        const password = getPassword();

        const response = await fetch('/create_user', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name, username, email, password})
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`); //Improve handling
        }

        //const data = response
    } catch (error) {
        console.error('Error creating user:', error);
        // Improve handling
      }
  };

  return (
    <Center style={{ width: '100vw', height: '100vh'  }}>
      <Container size="sm" style={{ textAlign: 'center' }}>
        <h1>Create User</h1>
        <form onSubmit={handleCreateUser}>
          <TextInput
            label="Name"
            placeholder="Enter your name"
            onChange={(e) => setName(e.target.value)}
            value={name}
            id="name"
          />
          <TextInput
            label="Username"
            placeholder="Enter your username"
            onChange={(e) => setUsername(e.target.value)}
            value={username}
            id="username"
          />
          <TextInput
            label="Email"
            placeholder="Enter your email"
            onChange={(e) => setEmail(e.target.value)}
            value={email}
            id="email"
          />
          <PasswordStrength/>
          <Button type="submit">Submit</Button>
        </form>
      </Container>
    </Center>
  );
};

export default CreateUserPage;
