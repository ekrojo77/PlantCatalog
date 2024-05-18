import { useNavigate } from 'react-router-dom';
import { useForm } from 'react-hook-form';
import { Input } from '../../components/ui/input';
import * as Card from '../../components/ui/card';
import { Flex } from 'styled-system/jsx';
import { Stack } from 'styled-system/jsx';
import { Button } from '../../components/ui/button';
import { FormLabel } from '../../components/ui/form-label';
import Masthead from '../../components/common/masthead';
import { useAuth } from '../../components/context/AuthContext';
import { useState } from 'react';

interface LoginFormData {
  username: string;
  password: string;
}

const LoginPage = () => {
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
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

      const token: string = responseData.token;

      login({ username: data.username, token: token });

      // Handle successful login, e.g., navigate to dashboard
      navigate('/');
    } catch (error) {
      console.error('Error Logging in:', error);
      setLoginError('Failed to login. Please check your credentials.');
    }
  };

  return (
    <div style={{ width: '100vw', height: '100vh', backgroundColor: 'var(--light-grey)', display: 'flex', flexDirection: 'column' }}>
      <Masthead />
      <Flex
        justifyContent="center"
        alignItems="center"
        style={{ flex: '1 1 auto', display: 'flex' }}
      >
        <form onSubmit={handleSubmit(handleLogin)} style={{ width: '100%', maxWidth: '400px', padding: '2rem' }}>
          <Card.Root
            width="100%"
            alignItems="center"
            justifyContent="center"
            style={{
              margin: 'auto',
              padding: '2rem',
              boxShadow: '0 4px 8px rgba(0, 0, 0, 0.1)',
              borderRadius: '10px',
              color: 'var(--text-color)',
              backgroundColor: 'var(--off-white)',
            }}
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
                  {errors.username && <span style={{ color: 'red' }}>{errors.username.message}</span>}
                </Stack>
                <Stack gap="1.5">
                  <FormLabel htmlFor="password">Password</FormLabel>
                  <Input
                    id="password"
                    type="password"
                    placeholder="Password"
                    {...register('password', { required: 'Password is required' })}
                    disabled={isSubmitting}
                  />
                  {errors.password && <span style={{ color: 'red' }}>{errors.password.message}</span>}
                </Stack>
              </Stack>
            </Card.Body>
            <Card.Footer style={{ textAlign: 'center', marginTop: '1rem' }}>
              <Button type="submit" disabled={isSubmitting}>Log In</Button>
              <div style={{ marginTop: '1rem' }}>
                <a href="#" onClick={() => navigate('/create-user')}>Create User</a>
              </div>
            </Card.Footer>
          </Card.Root>
          {loginError && <div style={{ color: 'red', textAlign: 'center', marginTop: '1rem' }}>{loginError}</div>}
        </form>
      </Flex>
    </div>
  );
};

export default LoginPage;
