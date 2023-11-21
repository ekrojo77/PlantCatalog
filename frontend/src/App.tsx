import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { MantineProvider } from '@mantine/core';
import HomePage from './pages/HomePage/HomePage'; 
import CreateUserPage from './pages/CreateUserPage/CreateUserPage';

const App = () => {
  return (
    <MantineProvider>
      <Router>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/create-user" element={<CreateUserPage />} />
        </Routes>
      </Router>
    </MantineProvider>
  );
};

export default App;
