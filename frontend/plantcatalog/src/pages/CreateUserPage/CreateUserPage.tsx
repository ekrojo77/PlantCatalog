import React, { useState } from 'react';
import Input from '../../components/Input/Input';
import Button from '../../components/Button/Button';

const CreateUserPage: React.FC = () => {
  const [name, setName] = useState<string>('');
  const [username, setUsername] = useState<string>('');
  const [email, setEmail] = useState<string>('');
  const [password, setPassword] = useState<string>('');

  const handleCreateUser = async () => {
    try {
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

        const data = await response.json();
    } catch (error) {
        console.error('Error creating user:', error);
        // Improve handling
      }
  };

  return (
    <div>
      <h1>Create User</h1>
      <form onSubmit={handleCreateUser}>
        <div>
          <label htmlFor="name">Name:</label>
          <Input
            type="text"
            placeholder="Enter your name"
            onChange={(e) => setName(e.target.value)}
            value={name}
            id="name"
          />
        </div>
        <div>
          <label htmlFor="username">Username:</label>
          <Input
            type="text"
            placeholder="Enter your username"
            onChange={(e) => setUsername(e.target.value)}
            value={username}
            id="username"
          />
        </div>
        <div>
          <label htmlFor="email">Email:</label>
          <Input
            type="email"
            placeholder="Enter your email"
            onChange={(e) => setEmail(e.target.value)}
            value={email}
            id="email"
          />
        </div>
        <div>
          <label htmlFor="password">Password:</label>
          <Input
            type="password"
            placeholder="Enter your password"
            onChange={(e) => setPassword(e.target.value)}
            value={password}
            id="password"
          />
        </div>
        <button type="submit">Submit</button>
      </form>
    </div>
  );
};

export default CreateUserPage;
