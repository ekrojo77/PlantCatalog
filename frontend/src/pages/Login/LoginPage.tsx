import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useForm } from 'react-hook-form';
import { Input } from '../../components/common/input';
import * as Card from '../../components/common/card';
import { Flex } from '../../../styled-system/jsx';
import { Stack } from '../../../styled-system/jsx';
import { Button } from '../../components/common/button';
import { FormLabel } from '../../components/common/form-label';
import Masthead from '../../components/common/masthead';
import { useAuth } from '../../hooks/useAuth';


interface LoginFormData {
  username: string;
  password: string;
}

const LoginPage: React.FC = () => {
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting},
  } = useForm<LoginFormData>();
  const [loginError, setLoginError] = useState('');
  const { login } = useAuth();

  const handleLogin = async (data: LoginFormData) => {
    setLoginError('');

    try {
      const response = await fetch('http://localhost:3000/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      
      const responseData = await response.json();

      const token = responseData.token;

      login({ username: data.username, authToken: token })

      // Handle successful login, e.g., navigate to dashboard
      navigate('/');
    } catch (error) {
      console.error('Error Logging in:', error);
      setLoginError('Failed to login. Please check your credentials.');
    } 
  };

  return (
    <Flex
      justifyContent="center"
      alignItems="center"
      style={{ width: '100vw', height: '100vh' }}
    >
     <Masthead /> {/* This is the header component we created earlier */}
      <form onSubmit={handleSubmit(handleLogin)}>
        <Card.Root 
          width="sm" 
          background='#FFFFFF' 
          alignItems="center" 
          justifyContent="center" 
          style={{ margin: 'auto'}}
        >
          <Card.Header>
            <Card.Title>Log In</Card.Title>
            <Card.Description>Log in to your user account.</Card.Description>
          </Card.Header>
          <Card.Body>
            <Stack gap="4">
              <Stack gap="1.5">
                <FormLabel htmlFor="username">Username</FormLabel>
                <Input 
                  id="username" 
                  placeholder="Name"
                  {...register('username', { required: 'Username is required' })} 
                  disabled={isSubmitting}  
                />
                {errors.username && <span>{errors.username.message}</span>}
              </Stack>
              <Stack gap="1.5">
                <FormLabel htmlFor="password">Password</FormLabel>
                <Input 
                  id="password" 
                  type="password" 
                  placeholder="password"
                  {...register('password', { required: 'Password is required' })} 
                  disabled={isSubmitting}
                />
                {errors.password && <span>{errors.password.message}</span>}
              </Stack>
            </Stack>
          </Card.Body>
          <Card.Footer gap="3">
            <Button type="submit" disabled={isSubmitting}>LogIn</Button>
            <Button variant="outline" onClick={() => navigate('/')}>Cancel</Button> 
          </Card.Footer>
        </Card.Root>
        {loginError && <div style={{ color: 'red', textAlign: 'center' }}>{loginError}</div>}
      </form>
    </Flex>
  );
};

export default LoginPage;
