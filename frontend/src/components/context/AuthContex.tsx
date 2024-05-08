// AuthContext.tsx
import React, { createContext, useState, useEffect, useContext, ReactNode } from 'react';


interface User {
  username: string;
}

interface AuthContextType {
  user: User | null;
  login: (userData: UserData) => void;
  logout: () => void;
}


interface UserData {
    username: string;
    token: string;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: ReactNode;
}

const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    const verifyToken = async () => {
      const userDataString = localStorage.getItem('user');
      if (userDataString) {
        const userData: UserData = JSON.parse(userDataString);
        const token = userData.token;

        try {
          const response = await fetch('http://localhost:3000/validate_token', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ token }),
          });
          console.log(response)
          if (response.ok) {
            setUser({ username: userData.username });
          } else {
            logout();
          }
        } catch (error) {
          console.error('Error verifying token:', error);
          logout();
        }
      }
    };

    verifyToken();
 }, []);

  const login = (userData: UserData) => {
    localStorage.setItem('user', JSON.stringify(userData));
    setUser({ username: userData.username }); 
  };

  const logout = () => {
    localStorage.removeItem('user');
    setUser(null);
  };

  const authContextValue: AuthContextType = {
    user,
    login,
    logout,
  };

  return <AuthContext.Provider value={authContextValue}>{children}</AuthContext.Provider>;
};

export { AuthProvider, AuthContext };
