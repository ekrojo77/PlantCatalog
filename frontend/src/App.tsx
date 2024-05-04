import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import HomePage from './pages/HomePage/HomePage';
import CreateUserPage from './pages/CreateUserPage/CreateUserPage';
import LoginPage from './pages/Login/LoginPage';
import { AuthContext } from './context/AuthContext';
import { useAuth } from './hooks/useAuth';

const App = () => {

  const { user, setUser} = useAuth();

  return (
    <AuthContext.Provider value={{ user, setUser}}>
      <Router>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/create-user" element={<CreateUserPage />} />
          <Route path="/login" element={<LoginPage />} />
        </Routes>
      </Router>
    </AuthContext.Provider>
  );
};

export default App;
