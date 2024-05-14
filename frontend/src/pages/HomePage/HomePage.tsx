import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Flex, Box } from 'styled-system/jsx';
import { Text } from '../../components/ui/text';
import Masthead from '../../components/common/masthead';
import { useAuth } from '../../components/context/AuthContext';
import image from './../../assets/images/homepage.webp';

const HomePage: React.FC = () => {
  const navigate = useNavigate();
  const { user } = useAuth();

  useEffect(() => {
    if (user) {
      navigate('/userpage');
    }
  }, [user, navigate]);

  return (
    <Flex
      direction="column"
      style={{ width: '100vw', height: '100vh', overflow: 'hidden' }} // Prevent overflow
    >
      <Masthead /> {/* This is the header component we created earlier */}
      {!user && (
        <Flex
          direction="row"
          style={{ flex: 1, height: 'calc(100vh - 60px)', display: 'flex' }}
        >
          <Box style={{ flex: '0 0 30%', height: '100%', overflow: 'hidden' }}>
            <img 
              src={image} 
              alt="Plants" 
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
                width: '40%', // Adjust width to prevent overflow
                padding: '40px',
                backgroundColor: 'var(--off-white)',
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'center',
                alignItems: 'center',
                boxShadow: '0 4px 8px rgba(0, 0, 0, 0.1)',
              }}
            >
              <Text size="2xl" fontWeight="bold">Welcome to PlantCatalog</Text>
              <Text size="lg" marginTop="10px">Your personal plant collection manager</Text>
              <Box marginTop="20px">
                <Text fontSize="md">Track and catalog your plants easily with PlantCatalog.</Text>
                <Text fontSize="md">Enjoy features like:</Text>
                <ul style={{ listStyleType: 'none', padding: 0, textAlign: 'left' }}>
                  <li>🌱 Detailed plant profiles</li>
                  <li>📸 Photo uploads</li>
                  <li>🔔 Maintenance reminders</li>
                </ul>
              </Box>
            </Box>
          </Flex>
        </Flex>
      )}
    </Flex>
  );
};

export default HomePage;
