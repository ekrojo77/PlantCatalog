import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import HomePage from './pages/HomePage/HomePage';
import CreateUserPage from './pages/CreateUserPage/CreateUserPage';
import LoginPage from './pages/Login/LoginPage';
import { AuthProvider } from './components/context/AuthContext';
import UserPage from './pages/UserPage/UserPage';

const App = () => {
  return (
    <AuthProvider>
      <Router>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/create-user" element={<CreateUserPage />} />
          <Route path="/login" element={<LoginPage />} />
          <Route path="/userpage" element={<UserPage />} />
        </Routes>
      </Router>
    </AuthProvider>
  );
};

export default App;
