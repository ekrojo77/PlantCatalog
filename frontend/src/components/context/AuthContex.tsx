import React, { createContext, useState, useEffect, useContext, ReactNode, useCallback } from 'react';

interface User {
  username: string;
}

interface AuthContextType {
  user: User | null;
  login: (userData: UserData) => void;
  logout: () => void;
  refreshAccessToken: () => Promise<void>;
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

  const refreshAccessToken = useCallback(async () => {
    const userDataString = localStorage.getItem('user');
    if (userDataString) {
      const userData: UserData = JSON.parse(userDataString);
      const token = userData.token;

      const tokenExp = JSON.parse(atob(token.split('.')[1])).exp;
      const now = Math.floor(Date.now() / 1000);
      if (tokenExp - now < 300) { // less than 5 minutes left
        try {
          const response = await fetch('http://localhost:3000/refresh_token', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ token }),
          });

          if (response.ok) {
            const data = await response.json();
            userData.token = data.access_token;
            localStorage.setItem('user', JSON.stringify(userData));
            setUser({ username: userData.username });
          } else {
            logout();
          }
        } catch (error) {
          console.error('Error refreshing access token:', error);
          logout();
        }
      }
    }
  }, []);

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
          if (response.ok) {
            setUser({ username: userData.username });
            await refreshAccessToken();
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

    const interval = setInterval(() => {
      refreshAccessToken();
    }, 5 * 60 * 1000); 

    return () => clearInterval(interval);
  }, [refreshAccessToken]);

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
    refreshAccessToken,
  };

  return <AuthContext.Provider value={authContextValue}>{children}</AuthContext.Provider>;
};

export { AuthProvider, AuthContext };
