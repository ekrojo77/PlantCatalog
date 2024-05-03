import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import HomePage from './pages/HomePage/HomePage';
import CreateUserPage from './pages/CreateUserPage/CreateUserPage';
import LoginPage from './pages/Login/LoginPage';
import { AuthContext } from './context/AuthContext';

const App = () => {
  return (
    <AuthContext.Provider value={{ user: null, setUser: () => {} }}>
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
