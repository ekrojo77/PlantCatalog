import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Flex, Box } from '../../../styled-system/jsx';
import Masthead from '../../components/common/masthead';
import { useAuth } from '../../components/context/AuthContext';
import { Text } from '../../components/ui/text';
import userImage from './../../assets/images/homepage.webp'; // Placeholder image for user page

const UserPage: React.FC = () => {
  const navigate = useNavigate();
  const { user } = useAuth();

  useEffect(() => {
    if (!user) {
      navigate('/');
    }
  }, [user, navigate]);

  return (
    <Flex
      direction="column"
      style={{ width: '100vw', height: '100vh', overflow: 'hidden' }} // Prevent overflow
    >
      <Masthead /> {/* This is the header component we created earlier */}
      <Flex
        direction="row"
        style={{ flex: 1, height: 'calc(100vh - 60px)', display: 'flex' }} // Ensure proper height and display
      >
        <Box style={{ flex: '0 0 30%', height: '100%', overflow: 'hidden' }}>
          <img 
            src={userImage} 
            alt="User" 
            style={{ width: '100%', height: '100%', objectFit: 'cover' }} 
          />
        </Box>
        <Flex
          style={{
            flex: 1,
            padding: '20px',
            backgroundColor: 'var(--light-grey)',
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            overflow: 'hidden', // Prevent inner overflow
          }}
        >
          <Box
            style={{
              width: '80%', // Adjust width to prevent overflow
              padding: '40px',
              backgroundColor: 'white',
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'center',
              alignItems: 'center',
              boxShadow: '0 4px 8px rgba(0, 0, 0, 0.1)',
            }}
          >
            <Text fontSize="2xl" fontWeight="bold">Welcome, {user?.username}</Text>
            <Text fontSize="lg" marginTop="10px">Here are your plants:</Text>
            <Box marginTop="20px" style={{ width: '100%' }}>
              {/* Placeholder for user plants. Replace with your actual content */}
              <Text fontSize="md">Plant 1: Fern</Text>
              <Text fontSize="md">Plant 2: Cactus</Text>
              <Text fontSize="md">Plant 3: Succulent</Text>
              {/* Add more plant details or other user-specific information here */}
            </Box>
          </Box>
        </Flex>
      </Flex>
    </Flex>
  );
};

export default UserPage;