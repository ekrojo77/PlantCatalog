import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useForm } from 'react-hook-form';
import { Input } from '../../components/common/input';
import { Flex } from '../../../styled-system/jsx';
import { Button } from '../../components/common/button';

interface LoginFormData {
  username: string;
  password: string;
}

const LoginPage: React.FC = () => {
  const navigate = useNavigate();
  const { register, handleSubmit, formState: { errors } } = useForm<LoginFormData>();
  const [loading, setLoading] = useState(false);
  const [loginError, setLoginError] = useState('');

  const handleLogin = async (data: LoginFormData) => {
    setLoading(true);
    setLoginError('');

    try {
      const response = await fetch('/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data)
      });

      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      // Handle successful login, e.g., navigate to dashboard
      navigate('/dashboard');
    } catch (error) {
      console.error('Error Logging in:', error);
      setLoginError('Failed to login. Please check your credentials.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Flex justifyContent="center" alignItems="center" style={{ height: '100vh', width: '100vw', display: 'flex' }}>
      <form onSubmit={handleSubmit(handleLogin)} style={{ display: 'flex', flexDirection: 'column', gap: '10px', width: '300px' }}>
        <label htmlFor="username">Username</label>
        <Input
          id="username"
          {...register('username', { required: 'Username is required' })}
          disabled={loading}
        />
        {errors.username && <span>{errors.username.message || 'Error'}</span>}


        <label htmlFor="password">Password</label>
        <Input
          id="password"
          type="password"
          {...register('password', { required: 'Password is required' })}
          disabled={loading}
        />
        {errors.password && <span>{errors.password.message || 'Error'}</span>}


        {loginError && <div style={{ color: 'red' }}>{loginError}</div>}

        <Button type="submit" disabled={loading}>
          {loading ? 'Logging in...' : 'Login'}
        </Button>
      </form>
    </Flex>
  );
};

export default LoginPage;
