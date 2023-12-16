import React, {useState} from 'react';
import { useForm } from 'react-hook-form';
import { Button } from '../../components/common/button';
import { Input } from '../../components/common/input';
import { Flex } from '../../../styled-system/jsx/flex';
import { Box } from '../../../styled-system/jsx/box';
import { Heading } from '../../components/common/heading';
import { Label } from '../../components/common/label';

interface FormData {
  name: string;
  username: string;
  email: string;
  password: string;
}

const CreateUserPage: React.FC = () => {
  const { register, handleSubmit, formState: { errors } } = useForm<FormData>();
  const [loading, setLoading] = useState(false);
  const [serverError, setServerError] = useState('');

  const handleCreateUser = async (data: FormData) => {
    setLoading(true);
    setServerError('');

    try {
      const response = await fetch('/create_user', {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
      });

      if (!response.ok) {
          throw new Error(`Server responded with status: ${response.status}`);
      }

      // Handle successful response
    } catch (error) {
      console.error('Error creating user:', error);
      if (error instanceof Error) {
        setServerError(error.message);
      } else {
        setServerError('An error occurred while creating the user.');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <Flex justifyContent="center" alignItems="center" style={{ minHeight: '100vh', width: '100vw', display: 'flex' }}>
      <Box width="400px" padding="20px" style={{ margin: 'auto' }}>
        <Heading as="h1" mb="4">Create User</Heading>
        <form onSubmit={handleSubmit(handleCreateUser)} style={{ display: 'flex', flexDirection: 'column' }}>
          <Label>Name</Label>
          <Input
            id="name"
            placeholder="Enter your name"
            {...register('name', { required: 'Name is required', minLength: { value: 3, message: 'Name must be at least 3 characters long' } })}
            aria-invalid={errors.name ? "true" : "false"}
            mb="2"
          />
          {errors.name && <span>This field is required</span>}

          <Label>Username</Label>
          <Input
            id="username"
            placeholder="Enter your username"
            {...register('username', { required: 'Username is required', pattern: { value: /^[A-Za-z0-9]+$/, message: 'Username must contain only letters and numbers' } })}
            aria-invalid={errors.name ? "true" : "false"}
            mb="2"
          />
          {errors.username && <span>This field is required</span>}

          <Label>Email</Label>
          <Input
            id="email"
            placeholder="Enter your email"
            {...register('email', { required: 'Email is required', pattern: { value: /^\S+@\S+\.\S+$/, message: 'Please enter a valid email address' } })}
            aria-invalid={errors.name ? "true" : "false"}
            mb="2"
          />
          {errors.email && <span>This field is required</span>}

          <Label>Password</Label>
          <Input
            id="password"
            placeholder="Enter your password"
            type="password"
            {...register('password', { required: 'Password is required', minLength: { value: 6, message: 'Password must be at least 6 characters long' } })}
            aria-invalid={errors.name ? "true" : "false"}
            mb="2"
          />
          {errors.password && <span>This field is required</span>}

          {serverError && <div style={{ color: 'red' }}>{serverError}</div>}

          <Button type="submit" disabled={loading}>
            {loading ? 'Creating...' : 'Submit'}
          </Button>
        </form>
      </Box>
    </Flex>
  );
};


export default CreateUserPage;
