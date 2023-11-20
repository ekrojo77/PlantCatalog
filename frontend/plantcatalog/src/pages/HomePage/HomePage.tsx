import React from 'react';
import { useNavigate  } from 'react-router-dom';
import Button from '../../components/Button/Button';

const HomePage: React.FC = () => {
  const navigate = useNavigate ();

  const navigateToCreateUser = () => {
    navigate('/create-user');
  };

  return (
    <div>
      <h1>Welcome to Our Application</h1>
      <Button onClick={navigateToCreateUser}>Create User</Button>
    </div>
  );
};

export default HomePage;
